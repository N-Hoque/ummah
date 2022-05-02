pub mod types;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono_utilities::naive::DateTransitions;
use clap::Parser;
use serde::{Deserialize, Serialize};
use types::{
    AdhanError, AdhanResult, AsrCalculationMethod, Kind, LatitudeMethod, PrayerCalculationMethod,
};

pub static LINK: &str = "https://www.salahtimes.com/uk/bath/csv";

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
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    kind: Kind,
    time: NaiveTime,
}

impl std::fmt::Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::Fajr => write!(f, "Fajr: {}", self.time),
            Kind::Dhuhr => write!(f, "Dhuhr: {}", self.time),
            Kind::Asr => write!(f, "Asr: {}", self.time),
            Kind::Maghrib => write!(f, "Maghrib: {}", self.time),
            Kind::Isha => write!(f, "Isha: {}", self.time),
        }
    }
}

impl Prayer {
    fn new(kind: Kind, time: NaiveTime) -> Self {
        Self { kind, time }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Day {
    date: NaiveDate,
    prayers: [Prayer; 5],
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = self.date.format("%A, %d %B %Y").to_string();

        output = format!("\n{:^62}\n", output);

        output += &format!("|{:=<62}|\n|", "");

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.kind.to_string());
            if idx < 4 {
                output += " | ";
            }
        }

        output += "|\n|";

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.time.to_string());
            if idx < 4 {
                output += " | ";
            }
        }

        output += &format!("|\n|{:=<62}|\n", "");

        write!(f, "{output}")
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl Day {
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_prayers(&self) -> [Prayer; 5] {
        self.prayers
    }
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

struct PrayerQueryBuilder {
    high_latitude_method: LatitudeMethod,
    prayer_calculation_method: PrayerCalculationMethod,
    asr_calculation_method: AsrCalculationMethod,
    current_month: NaiveDate,
}

impl PrayerQueryBuilder {
    fn build(self) -> String {
        let current_year = self.current_month.year();
        let current_month = self.current_month.month();
        let end_day = self.current_month.last_day_of_month();

        let start_date = format!("{}-{}-01", current_year, current_month);
        let end_date = format!("{}-{}-{}", current_year, current_month, end_day);

        format!(
            "{}?highlatitudemethod={}&prayercalculationmethod={}&asarcalculationmethod={}&start={}&end={}",
            LINK,
            self.high_latitude_method as u8,
            self.prayer_calculation_method as u8,
            self.asr_calculation_method as u8,
            start_date,
            end_date
        )
    }
}

#[derive(Debug, Deserialize)]
struct CSVPrayer {
    day: String,
    fajr: String,
    _s: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

impl CSVPrayer {
    fn build(self) -> AdhanResult<Day> {
        let year = Utc::now().year();

        let fajr = format!("{} {}, {}", self.day, year, self.fajr);
        let dhuhr = format!("{} {}, {}", self.day, year, self.dhuhr);
        let asr = format!("{} {}, {}", self.day, year, self.asr);
        let maghrib = format!("{} {}, {}", self.day, year, self.maghrib);
        let isha = format!("{} {}, {}", self.day, year, self.isha);

        let fajr = NaiveDateTime::parse_from_str(&fajr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let dhuhr = NaiveDateTime::parse_from_str(&dhuhr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let asr = NaiveDateTime::parse_from_str(&asr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let maghrib = NaiveDateTime::parse_from_str(&maghrib, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let isha = NaiveDateTime::parse_from_str(&isha, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;

        let rhs = Duration::hours(12);
        Ok(Day {
            date: fajr.date(),
            prayers: [
                Prayer::new(Kind::Fajr, fajr.time()),
                Prayer::new(Kind::Dhuhr, dhuhr.time().overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Asr, asr.time().overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Maghrib, maghrib.time().overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Isha, isha.time().overflowing_add_signed(rhs).0),
            ],
        })
    }
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
