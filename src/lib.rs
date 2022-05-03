pub(crate) mod day;
pub(crate) mod prayer;
pub(crate) mod request_parser;
pub mod types;

use chrono::{Datelike, Local};
use clap::Parser;
use day::Day;
use request_parser::{csv_parser::CSVPrayer, query_builder::PrayerQueryBuilder};
use serde::{Deserialize, Serialize};
use types::{
    AdhanError, AdhanResult, AsrCalculationMethod, LatitudeMethod, PrayerCalculationMethod,
};

use std::fs::File;

/// Gets prayer times from www.salahtimes.com
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct PrayerArguments {
    /// Latitude method
    #[clap(short, long, arg_enum, default_value = "one-seventh")]
    latitude_method: LatitudeMethod,

    /// Source of Prayer calculation
    #[clap(short, long, arg_enum, default_value = "mwl")]
    prayer_method: PrayerCalculationMethod,

    /// Asr time method
    #[clap(short, long, arg_enum, default_value = "shafi")]
    asr_method: AsrCalculationMethod,

    /// Get today's times
    #[clap(short, long)]
    today_only: bool,
}

impl PrayerArguments {
    pub fn settings(&self) -> PrayerSettings {
        PrayerSettings {
            latitude_method: self.latitude_method,
            prayer_method: self.prayer_method,
            asr_method: self.asr_method,
            current_month: Local::now().month(),
        }
    }

    pub fn is_today_only(&self) -> bool {
        self.today_only
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayerSettings {
    latitude_method: LatitudeMethod,
    prayer_method: PrayerCalculationMethod,
    asr_method: AsrCalculationMethod,
    current_month: u32,
}

impl PrayerSettings {
    fn query(&self) -> String {
        PrayerQueryBuilder {
            high_latitude_method: self.latitude_method,
            prayer_calculation_method: self.prayer_method,
            asr_calculation_method: self.asr_method,
            current_month: Local::now().naive_utc().date(),
        }
        .build()
    }
}

pub async fn get_prayer_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    if check_settings(prayer_settings) {
        from_yaml()
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
