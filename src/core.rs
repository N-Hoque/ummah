pub(crate) mod fs;
pub(crate) mod html_creator;
pub(crate) mod request_handler;

use self::{
    fs::{get_cache_filepath, get_user_filepath, open_file, write_file, write_serialized_file},
    html_creator::{create_table, create_title, generate_default_css, generate_template_css},
    request_handler::download_file,
};

use crate::{
    day::Day,
    prayer::settings::PrayerSettings,
    request_parser::parse_csv_file,
    types::AdhanResult,
};

use bytes::Bytes;
use chrono::Local;
use html_builder::Html5;

use std::path::PathBuf;

static CURRENT_MONTH: &str = "current_month.yaml";
static CURRENT_SETTINGS: &str = ".current_settings.yaml";
static CURRENT_HTML: &str = "current_month.html";

static ADHAN_MP3_LINK: &str = "https://media.sd.ma/assabile/adhan_3435370/8c052a5edec1.mp3";

/// Collect all prayer times for the current month
///
/// # Example
/// ```
/// use adhan::{
///     core::{get_prayer_times, try_get_today},
///     prayer::settings::PrayerSettings,
///     types::{AdhanResult, LatitudeMethod, PrayerCalculationMethod, AsrCalculationMethod}
/// };
///
/// async fn test() -> AdhanResult<()> {
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
pub async fn get_prayer_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    match (check_settings(prayer_settings), load_data()) {
        (true, Some(month)) => Ok(month),
        _ => request_times(prayer_settings).await,
    }
}

/// Show the prayer times for today
///
/// # Example
/// ```
/// use adhan::{
///     core::{get_prayer_times, try_get_today},
///     prayer::settings::PrayerSettings,
///     types::{AdhanResult, LatitudeMethod, PrayerCalculationMethod, AsrCalculationMethod}
/// };
///
/// async fn test() -> AdhanResult<()> {
///     let settings = PrayerSettings::new(LatitudeMethod::OneSeventh, PrayerCalculationMethod::MWL, AsrCalculationMethod::Shafi);
///     let month = get_prayer_times(&settings).await?;
///     assert!(!month.is_empty());
///
///     let today = try_get_today(&month);
///
///     if let Some(today) = today {
///         println!("{}", today);
///     }
///
///     Ok(())
/// }
/// ```
pub fn try_get_today(month: &[Day]) -> Option<&Day> {
    let today = Local::now().date().naive_utc();
    let today = month.iter().find(|day| day.get_date() == today);
    today
}

/// Creates an HTML page for the prayer timetable
pub fn export_html(month: &[Day], generate_css: bool) -> AdhanResult<()> {
    let mut document = html_builder::Buffer::new();

    let mut html = document.html().attr("lang=en-gb");

    create_title(&mut html)?;
    create_table(&mut html, month)?;

    let final_document = document.finish();

    let user_path = get_user_filepath();

    write_file(
        &user_path,
        &PathBuf::from(CURRENT_HTML),
        final_document.as_bytes(),
    )?;

    if generate_css {
        generate_default_css()?;
    } else {
        generate_template_css()?;
    }

    Ok(())
}

fn check_settings(prayer_settings: &PrayerSettings) -> bool {
    let path = get_cache_filepath().join(CURRENT_SETTINGS);
    match open_file(path) {
        Err(_) => false,
        Ok(file) => match serde_yaml::from_reader::<_, PrayerSettings>(file) {
            Err(_) => false,
            Ok(settings) => settings == *prayer_settings,
        },
    }
}

fn load_data() -> Option<Vec<Day>> {
    let path = get_user_filepath().join(CURRENT_MONTH);
    match open_file(path) {
        Err(_) => None,
        Ok(file) => serde_yaml::from_reader(file).ok(),
    }
}

async fn request_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    let timetable = download_file(prayer_settings.query(Local::now().date().naive_utc()), "Downloading times").await?;

    let days = parse_csv_file(timetable)?;

    let audio = download_file(ADHAN_MP3_LINK, "Downloading adhan").await?;

    cache_data(&days, audio, prayer_settings)?;

    Ok(days)
}

fn cache_data(days: &Vec<Day>, audio: Bytes, prayer_settings: &PrayerSettings) -> AdhanResult<()> {
    let (docs, cache) = (get_user_filepath(), get_cache_filepath());
    write_file(&docs, &PathBuf::from("adhan.mp3"), audio.as_ref())?;
    write_serialized_file(&docs, &PathBuf::from(CURRENT_MONTH), days)?;
    write_serialized_file(
        &cache,
        &PathBuf::from(CURRENT_SETTINGS),
        &prayer_settings.clone().with_audio_downloaded(),
    )?;

    Ok(())
}
