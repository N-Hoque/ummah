use crate::{
    day::Day,
    prayer::settings::PrayerSettings,
    request_parser::csv_parser::CSVPrayer,
    types::{AdhanError, AdhanResult},
};

use chrono::Local;

use serde::Serialize;

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

static CURRENT_MONTH: &str = "current_month.yaml";
static CURRENT_SETTINGS: &str = ".current_settings.yaml";

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

fn check_settings(prayer_settings: &PrayerSettings) -> bool {
    let path = get_cache_filepath()
        .map_or_else(|| CURRENT_SETTINGS.into(), |dir| dir.join(CURRENT_SETTINGS));
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
        get_month_filepath().map_or_else(|| CURRENT_MONTH.into(), |dir| dir.join(CURRENT_MONTH));
    match open_file(path) {
        Err(_) => None,
        Ok(file) => serde_yaml::from_reader(file).ok(),
    }
}

async fn from_csv(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    print!("Loading times...\r");
    std::io::stdout()
        .flush()
        .map_err(|x| AdhanError::Request(Box::new(x)))?;

    let data = download_csv_file(prayer_settings).await?;

    print!("{:<17}\r", "");
    std::io::stdout()
        .flush()
        .map_err(|x| AdhanError::Request(Box::new(x)))?;

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
        .map_err(|x| AdhanError::Request(Box::new(x)))?;
    let content = response
        .text()
        .await
        .map_err(|x| AdhanError::Request(Box::new(x)))?;
    Ok(content)
}

fn cache_times(days: &Vec<Day>, prayer_settings: &PrayerSettings) -> AdhanResult<()> {
    if let (Some(docs), Some(cache)) = (get_month_filepath(), get_cache_filepath()) {
        write_file(&docs, &PathBuf::from(CURRENT_MONTH), days)?;
        write_file(&cache, &PathBuf::from(CURRENT_SETTINGS), &prayer_settings)?;
    } else {
        write_file("adhan", CURRENT_MONTH, days)?;
        write_file("adhan", CURRENT_SETTINGS, &prayer_settings)?;
    }

    Ok(())
}

fn get_month_filepath() -> Option<PathBuf> {
    dirs_next::document_dir().map(|dir| dir.join("adhan"))
}

fn get_cache_filepath() -> Option<PathBuf> {
    dirs_next::cache_dir().map(|dir| dir.join("adhan"))
}

fn open_file<P: AsRef<Path>>(path: P) -> AdhanResult<File> {
    File::open(path).map_err(AdhanError::IO)
}

fn write_file<P: AsRef<Path>, T: Serialize>(dir: P, file: P, data: &T) -> AdhanResult<()> {
    if std::fs::read_dir(&dir).is_err() {
        std::fs::create_dir_all(&dir).map_err(AdhanError::IO)?;
    }

    let mut file = File::create(dir.as_ref().join(file)).map_err(AdhanError::IO)?;
    serde_yaml::to_writer(&mut file, data).map_err(AdhanError::Serde)
}
