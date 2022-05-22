//! Core module for obtaining and caching timetable
//! and other relevant files

pub mod fs;
pub mod prayer;
pub(crate) mod request_handler;
pub mod timetable_generator;

use self::{
    fs::{get_cache_filepath, get_user_filepath, open_file, write_serialized_file},
    request_handler::download_file,
};

use crate::{
    argparser::settings::PrayerSettings,
    request_parser::parse_csv_file,
    time::{day::Day, month::Month},
    types::UmmahResult,
};

use chrono::{Datelike, Local};

use std::path::PathBuf;

static CURRENT_MONTH: &str = "current_month.yaml";
static CURRENT_SETTINGS: &str = ".current_settings.yaml";

/// Collect all prayer times for the current month
///
/// # Example
/// ```
/// use ummah::{
///     core::{get_prayer_times, try_get_today},
///     prayer::settings::PrayerSettings,
///     types::{UmmahResult, LatitudeMethod, PrayerCalculationMethod, AsrCalculationMethod}
/// };
///
/// async fn test() -> UmmahResult<()> {
///     let settings = PrayerSettings::new(LatitudeMethod::OneSeventh, PrayerCalculationMethod::MWL, AsrCalculationMethod::Shafi);
///     let month = get_prayer_times(&settings).await?;
///     assert!(!month.is_empty());
///
///     // Print all days in the month
///     for day in month {
///         println!("{}", day);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn get_prayer_times(
    prayer_settings: &PrayerSettings,
    custom_month: Option<u32>,
) -> UmmahResult<Month> {
    match (check_settings(prayer_settings), load_data(), custom_month) {
        (_, _, Some(custom_month)) => request_times(prayer_settings, custom_month).await,
        (true, Some(month), _) => Ok(month),
        _ => request_times_now(prayer_settings).await,
    }
}

/// Deletes all cached data
///
/// NB: Cached data is stored in the documents and cache directories.
/// This directory differs between OSes.
///  
/// - [Documents](https://docs.rs/dirs-next/2.0.0/dirs_next/fn.document_dir.html)
/// - [Cache](https://docs.rs/dirs-next/2.0.0/dirs_next/fn.cache_dir.html)
pub fn clear_cache() -> UmmahResult<()> {
    let (docs, cache) = (get_user_filepath(), get_cache_filepath());

    std::fs::remove_dir_all(docs)?;
    std::fs::remove_dir_all(cache)?;

    Ok(())
}

/// Updates the timetable for a given day
pub fn update_timetable(day: &Day) -> UmmahResult<()> {
    let mut month = load_data().expect("Loading timetable");

    month.update_day(day);

    let docs = get_user_filepath();

    write_serialized_file(&docs, &PathBuf::from(CURRENT_MONTH), &month)
}

pub fn get_performed_status(date: chrono::NaiveDate, prayer_time: chrono::NaiveTime) -> bool {
    let current_datetime = Local::now();
    let current_date = current_datetime.date().naive_local();
    let current_time = current_datetime.time();

    match date.cmp(&current_date) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Equal => prayer_time <= current_time,
    }
}

fn check_settings(prayer_settings: &PrayerSettings) -> bool {
    let path = get_cache_filepath().join(CURRENT_SETTINGS);
    match open_file(path) {
        Err(_) => false,
        Ok(file) => match serde_yaml::from_reader::<_, PrayerSettings>(file) {
            Err(_) => false,
            Ok(settings) => settings == prayer_settings.clone().with_audio_downloaded(),
        },
    }
}

fn load_data() -> Option<Month> {
    let path = get_user_filepath().join(CURRENT_MONTH);
    open_file(path)
        .ok()
        .and_then(|file| serde_yaml::from_reader::<_, Month>(file).ok())
}

async fn request_times_now(prayer_settings: &PrayerSettings) -> UmmahResult<Month> {
    let timetable = download_file(
        prayer_settings.query(Local::now().date().naive_utc()),
        "Downloading times",
    )
    .await?;

    let month = parse_csv_file(timetable)?;

    cache_data(&month, prayer_settings)?;

    Ok(month)
}

async fn request_times(prayer_settings: &PrayerSettings, month: u32) -> UmmahResult<Month> {
    let timetable = download_file(
        prayer_settings.query(chrono::NaiveDate::from_ymd(Local::now().year(), month, 1)),
        "Downloading times",
    )
    .await?;

    let month = parse_csv_file(timetable)?;

    cache_data(&month, prayer_settings)?;

    Ok(month)
}

fn cache_data(days: &Month, prayer_settings: &PrayerSettings) -> UmmahResult<()> {
    let (docs, cache) = (get_user_filepath(), get_cache_filepath());
    write_serialized_file(&docs, &PathBuf::from(CURRENT_MONTH), days)?;
    write_serialized_file(
        &cache,
        &PathBuf::from(CURRENT_SETTINGS),
        &prayer_settings.clone().with_audio_downloaded(),
    )?;

    Ok(())
}
