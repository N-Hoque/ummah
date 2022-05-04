pub(crate) mod fs;
pub(crate) mod html_creator;

use self::{
    fs::{get_cache_filepath, get_user_filepath, open_file, write_file, write_serialized_file},
    html_creator::{create_table, create_title, generate_default_css},
};

use crate::{
    day::Day,
    prayer::settings::PrayerSettings,
    request_parser::csv_parser::CSVPrayer,
    types::{AdhanError, AdhanResult},
};

use chrono::Local;
use html_builder::Html5;

use std::{io::Write as IOWrite, path::PathBuf};

static CURRENT_MONTH: &str = "current_month.yaml";
static CURRENT_SETTINGS: &str = ".current_settings.yaml";
static CURRENT_HTML: &str = "current_month.html";

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
    match (check_settings(prayer_settings), from_yaml()) {
        (true, Some(month)) => Ok(month),
        _ => from_csv(prayer_settings).await,
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

pub fn export_html(month: &[Day]) -> AdhanResult<()> {
    let mut document = html_builder::Buffer::new();

    let mut html = document.html().attr("lang=en-gb");

    create_title(&mut html)?;
    create_table(&mut html, month)?;

    let final_document = document.finish();

    let user_path = get_user_filepath();

    write_file(&user_path, &PathBuf::from(CURRENT_HTML), final_document)?;
    generate_default_css()?;

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

fn from_yaml() -> Option<Vec<Day>> {
    let path =
        get_user_filepath().join(CURRENT_MONTH);
    match open_file(path) {
        Err(_) => None,
        Ok(file) => serde_yaml::from_reader(file).ok(),
    }
}

async fn from_csv(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    print!("Loading times...\r");
    std::io::stdout()
        .flush()
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;

    let data = download_csv_file(prayer_settings).await?;

    print!("{:<17}\r", "");
    std::io::stdout()
        .flush()
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;

    let mut csv_reader = csv::Reader::from_reader(data.as_bytes());

    let mut days = vec![];
    for record in csv_reader.records() {
        let day = record
            .and_then(|x| x.deserialize::<'_, CSVPrayer>(None))
            .map_err(AdhanError::CSV)?
            .build()?;
        days.push(day);
    }

    cache_times(&days, prayer_settings)?;

    Ok(days)
}

async fn download_csv_file(prayer_settings: &PrayerSettings) -> AdhanResult<String> {
    let response = reqwest::get(prayer_settings.query())
        .await
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    let content = response
        .text()
        .await
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    Ok(content)
}

fn cache_times(days: &Vec<Day>, prayer_settings: &PrayerSettings) -> AdhanResult<()> {
    let (docs, cache) = (get_user_filepath(), get_cache_filepath());
    write_serialized_file(&docs, &PathBuf::from(CURRENT_MONTH), days)?;
    write_serialized_file(&cache, &PathBuf::from(CURRENT_SETTINGS), &prayer_settings)?;

    Ok(())
}
