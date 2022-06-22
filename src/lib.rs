//! # Ummah
//!
//! ## Overview
//!
//! Ummah is a library for obtaining the prayer times from [www.salahtimes.com/uk](www.salahtimes.com/uk)
//!
//! It provides support for settings prayer time calculations for
//! different schools of thought.

pub mod commands;
pub mod fs;
pub mod prayer;
pub(crate) mod request;
pub mod time;
pub mod types;

pub use crate::{
    prayer::Prayer,
    types::{PrayerName, UmmahError, UmmahResult},
};

use crate::time::{day::Day, month::Month};

use self::{
    fs::{get_cache_filepath, get_user_filepath, open_file, write_serialized_file},
    request::handler::download_file,
};

use commands::settings::PrayerSettings;
use request::parser::parse_csv_file;

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

use std::path::PathBuf;

static CURRENT_MONTH: &str = "current_month.yaml";
static CURRENT_SETTINGS: &str = ".current_settings.yaml";

/// Collect all prayer times for the current month
///
/// # Example
/// ```
/// use ummah::{
///     commands::settings::{PrayerSettings, CalculationMethods, Location},
///     types::{UmmahResult, LatitudeMethod, Organisation, AsrMethod},
///     get_prayer_times,
/// };
///
/// async fn test() -> UmmahResult<()> {
///     let settings = PrayerSettings::new(
///         CalculationMethods {
///             latitude: LatitudeMethod::OneSeventh,
///             organisation: Organisation::MWL,
///             asr: AsrMethod::Shafi
///         },
///         Location {country: "uk".into(), city: "bath".into()}
///     );
///     let month = get_prayer_times(&settings, None).await?;
///
///     // Print all days in the month
///     for day in month.iter() {
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

pub fn get_performed_status(
    current_date: NaiveDateTime,
    prayer_date: NaiveDate,
    prayer_time: NaiveTime,
) -> bool {
    let date = current_date.date();
    let time = current_date.time();

    match prayer_date.cmp(&date) {
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => prayer_time <= time,
    }
}

/// Generate an array of test prayers.
///
/// NB: These are not valid prayer times. This is for testing only.
pub fn create_test_prayers() -> [Prayer; 5] {
    [
        Prayer::new(
            PrayerName::Fajr,
            chrono::NaiveTime::from_hms(0, 0, 0),
            false,
        ),
        Prayer::new(
            PrayerName::Dhuhr,
            chrono::NaiveTime::from_hms(0, 0, 10),
            false,
        ),
        Prayer::new(
            PrayerName::Asr,
            chrono::NaiveTime::from_hms(0, 0, 20),
            false,
        ),
        Prayer::new(
            PrayerName::Maghrib,
            chrono::NaiveTime::from_hms(0, 0, 30),
            false,
        ),
        Prayer::new(
            PrayerName::Isha,
            chrono::NaiveTime::from_hms(0, 0, 40),
            false,
        ),
    ]
}

/// Creates a test day.
///
/// Uses the [create_test_prayers] function for the prayers of the test day
///
/// The test date is always January 1st, 2022
pub fn create_test_day() -> Day {
    Day::new(
        chrono::NaiveDate::from_ymd(2022, 1, 1),
        create_test_prayers(),
    )
}

/// Creates a test month.
///
/// Takes in `month` value from 1 - 12 and generates each day using [create_test_prayers].
///
/// The test month starts in the year 2022. This can be overridden with an optional year parameter
pub fn create_test_month(month: u32, year: Option<i32>) -> Month {
    let year = year.unwrap_or(2022);

    let is_leap_year = year % 400 == 0 || (year % 4 == 0 && year % 100 != 0);

    let max_days = match month {
        2 if is_leap_year => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    let days = (1..=max_days)
        .map(|day| {
            Day::new(
                chrono::NaiveDate::from_ymd(year, month, day),
                create_test_prayers(),
            )
        })
        .collect();

    Month::new(days)
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
        prayer_settings.query(NaiveDate::from_ymd(Local::now().year(), month, 1)),
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

#[cfg(test)]
mod tests;
