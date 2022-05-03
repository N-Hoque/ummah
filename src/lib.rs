pub mod arguments;
pub(crate) mod day;
pub(crate) mod prayer;
pub(crate) mod request_parser;
pub mod types;

use chrono::Local;

use day::Day;
use prayer::settings::PrayerSettings;
use request_parser::csv_parser::CSVPrayer;
use serde::Serialize;
use types::{AdhanError, AdhanResult};

use std::fs::File;

pub async fn get_prayer_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    if let (true, Ok(month)) = (check_settings(prayer_settings), from_yaml()) {
        Ok(month)
    } else {
        from_csv(prayer_settings).await
    }
}

pub fn try_get_today(month: &[Day]) -> Option<&Day> {
    let today = Local::now().date().naive_utc();
    let today = month.iter().find(|day| day.get_date() == today);
    today
}

fn check_settings(prayer_settings: &PrayerSettings) -> bool {
    open_file(".current_settings.yaml").map_or_else(
        |_x| false,
        |x| {
            serde_yaml::from_reader::<_, PrayerSettings>(x)
                .map_or_else(|_x| false, |settings| settings == *prayer_settings)
        },
    )
}

fn from_yaml() -> AdhanResult<Vec<Day>> {
    let file = open_file("current_month.yaml")?;
    serde_yaml::from_reader(file).map_err(AdhanError::Serde)
}

async fn from_csv(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    let data = download_csv_file(prayer_settings).await?;

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

fn cache_times(days: &Vec<Day>, prayer_settings: &PrayerSettings) -> AdhanResult<()> {
    write_file("current_month.yaml", days)?;
    write_file(".current_settings.yaml", &prayer_settings)?;
    Ok(())
}

fn open_file(path: &str) -> AdhanResult<File> {
    File::open(path).map_err(AdhanError::File)
}

fn write_file<T: Serialize>(path: &str, data: &T) -> AdhanResult<()> {
    let mut file = File::create(path).map_err(AdhanError::File)?;
    serde_yaml::to_writer(&mut file, data).map_err(AdhanError::Serde)
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
