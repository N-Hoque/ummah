use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono_utilities::naive::DateTransitions;
use clap::Parser;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use types::{AdhanError, AsrCalculationMethod, Kind, LatitudeMethod, PrayerCalculationMethod};

pub mod types;

pub static DATA: &str = "https://www.salahtimes.com/uk/bath/csv";

/// Gets prayer times from www.salahtimes.com
#[derive(Parser, Debug)]
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
            DATA,
            self.high_latitude_method as u8,
            self.prayer_calculation_method as u8,
            self.asr_calculation_method as u8,
            start_date,
            end_date
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    kind: Kind,
    time: NaiveTime,
}

impl Serialize for Prayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Prayer", 2)?;
        match self.kind {
            Kind::Fajr => state.serialize_field("Fajr", &self.time)?,
            Kind::Dhuhr => state.serialize_field("Dhuhr", &self.time)?,
            Kind::Asr => state.serialize_field("Asr", &self.time)?,
            Kind::Maghrib => state.serialize_field("Maghrib", &self.time)?,
            Kind::Isha => state.serialize_field("Isha", &self.time)?,
        }
        state.end()
    }
}

impl std::fmt::Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Kind::Fajr => write!(f, "Fajr:    {}", self.time),
            Kind::Dhuhr => write!(f, "Dhuhr    {}", self.time),
            Kind::Asr => write!(f, "Asr:     {}", self.time),
            Kind::Maghrib => write!(f, "Maghrib: {}", self.time),
            Kind::Isha => write!(f, "Isha:    {}", self.time),
        }
    }
}

impl Prayer {
    fn new(kind: Kind, time: NaiveTime) -> Self {
        Self { kind, time }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Day {
    date: NaiveDate,
    prayers: [Prayer; 5],
}

impl Day {
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_prayers(&self) -> [Prayer; 5] {
        self.prayers
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = self.date.format("%A, %d %B %Y").to_string() + "\n\t";

        for (idx, prayer) in self.prayers.into_iter().enumerate() {
            output += &prayer.to_string();
            if idx < 4 {
                output += "\n\t";
            }
        }

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
    fn build(self) -> Result<Day, AdhanError> {
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

pub async fn from_csv(prayer_arguments: PrayerArguments) -> Result<Vec<Day>, AdhanError> {
    let data = download_csv_file(prayer_arguments).await?;

    let mut csv_reader = csv::Reader::from_reader(data.as_bytes());

    let mut days = vec![];
    for record in csv_reader.records() {
        let day = record
            .and_then(|x| x.deserialize::<'_, CSVPrayer>(None))
            .map_err(|_| AdhanError::Deserialize)?
            .build()?;
        days.push(day);
    }

    Ok(days)
}

pub fn export_to_yaml(month: Vec<Day>) -> Result<(), AdhanError> {
    let mut current_month =
        std::fs::File::create("current_month.yaml").map_err(AdhanError::FileCreation)?;

    serde_yaml::to_writer(&mut current_month, &month).map_err(AdhanError::SerializedFileWrite)
}

async fn download_csv_file(prayer_arguments: PrayerArguments) -> Result<String, AdhanError> {
    let response = reqwest::get(
        PrayerQueryBuilder {
            high_latitude_method: prayer_arguments.latitude_method,
            prayer_calculation_method: prayer_arguments.prayer_method,
            asr_calculation_method: prayer_arguments.asr_method,
            current_month: chrono::Local::now().naive_utc().date(),
        }
        .build(),
    )
    .await
    .map_err(|_| AdhanError::Request)?;
    let content = response.text().await.map_err(|_| AdhanError::Download)?;
    Ok(content)
}
