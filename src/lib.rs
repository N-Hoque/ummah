pub(crate) mod day;
pub(crate) mod prayer;
pub(crate) mod request_parser;
pub mod types;

use chrono::Datelike;
use clap::Parser;
use day::Day;
use request_parser::{CSVPrayer, PrayerQueryBuilder};
use serde::{Deserialize, Serialize};
use types::{
    AdhanError, AdhanResult, AsrCalculationMethod, LatitudeMethod, PrayerCalculationMethod,
};

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
            current_month: chrono::Local::now().month(),
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

pub async fn get_prayer_times(prayer_settings: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    if let Ok(month) = from_yaml(prayer_settings) {
        Ok(month)
    } else {
        from_csv(prayer_settings).await
    }
}

pub fn try_get_today(month: &[Day]) -> Option<&Day> {
    let today = chrono::Local::now().date().naive_utc();
    let today = month.iter().find(|day| day.get_date() == today);
    today
}

fn from_yaml(prayer_arguments: &PrayerSettings) -> AdhanResult<Vec<Day>> {
    let settings = open_file(".current_settings.yaml")?;

    let settings: PrayerSettings = serde_yaml::from_reader(settings).map_err(AdhanError::Serde)?;
    if settings == *prayer_arguments {
        let file = open_file("current_month.yaml")?;
        serde_yaml::from_reader(file).map_err(AdhanError::Serde)
    } else {
        println!("Settings have changed. Reloading...");
        Err(AdhanError::Request)
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

    write_file("current_month.yaml", &days)?;
    write_file(".current_settings.yaml", &prayer_settings)?;

    Ok(days)
}

fn open_file(path: &str) -> AdhanResult<std::fs::File> {
    std::fs::File::open(path).map_err(AdhanError::File)
}

fn write_file<T: Serialize>(path: &str, data: &T) -> AdhanResult<()> {
    let mut file = std::fs::File::create(path).map_err(AdhanError::File)?;
    serde_yaml::to_writer(&mut file, data).map_err(AdhanError::Serde)
}

async fn download_csv_file(prayer_settings: &PrayerSettings) -> AdhanResult<String> {
    let response = reqwest::get(
        PrayerQueryBuilder {
            high_latitude_method: prayer_settings.latitude_method,
            prayer_calculation_method: prayer_settings.prayer_method,
            asr_calculation_method: prayer_settings.asr_method,
            current_month: chrono::Local::now().naive_utc().date(),
        }
        .build(),
    )
    .await
    .map_err(|_| AdhanError::Request)?;
    let content = response.text().await.map_err(|_| AdhanError::Download)?;
    Ok(content)
}
