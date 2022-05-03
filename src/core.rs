use chrono::Local;

use crate::day::Day;
use crate::prayer::settings::PrayerSettings;
use crate::request_parser::csv_parser::CSVPrayer;
use crate::types::{AdhanError, AdhanResult};
use serde::Serialize;

use std::fs::File;

pub async fn get_prayer_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    match (check_settings(prayer_settings), from_yaml()) {
        (true, Some(month)) => Ok(month),
        _ => from_csv(prayer_settings).await,
    }
}

pub fn try_get_today(month: &[Day]) -> Option<&Day> {
    let today = Local::now().date().naive_utc();
    let today = month.iter().find(|day| day.get_date() == today);
    today
}

fn check_settings(prayer_settings: &PrayerSettings) -> bool {
    match open_file(".current_settings.yaml") {
        Err(_) => false,
        Ok(file) => match serde_yaml::from_reader::<_, PrayerSettings>(file) {
            Err(_) => false,
            Ok(settings) => settings == *prayer_settings,
        },
    }
}

fn from_yaml() -> Option<Vec<Day>> {
    match open_file("current_month.yaml") {
        Err(_) => None,
        Ok(file) => serde_yaml::from_reader(file).ok(),
    }
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
