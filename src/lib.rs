use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{ser::SerializeStruct, Deserialize, Serialize};

pub static DATA: &str = "https://www.salahtimes.com/uk/bath/csv";

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
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

    fn apply_asr_correction(self) -> Self {
        if let Kind::Asr = self.kind {
            Prayer::new(
                Kind::Asr,
                self.time.overflowing_sub_signed(Duration::hours(1)).0,
            )
        } else {
            self
        }
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
    fn build(self) -> Result<Day, Box<dyn std::error::Error>> {
        let year = Utc::now().year();

        let fajr = format!("{} {}, {}", self.day, year, self.fajr);
        let dhuhr = format!("{} {}, {}", self.day, year, self.dhuhr);
        let asr = format!("{} {}, {}", self.day, year, self.asr);
        let maghrib = format!("{} {}, {}", self.day, year, self.maghrib);
        let isha = format!("{} {}, {}", self.day, year, self.isha);

        let fajr = NaiveDateTime::parse_from_str(&fajr, "%a %d %b %Y, %k:%M")?;
        let dhuhr = NaiveDateTime::parse_from_str(&dhuhr, "%a %d %b %Y, %k:%M")?;
        let asr = NaiveDateTime::parse_from_str(&asr, "%a %d %b %Y, %k:%M")?;
        let maghrib = NaiveDateTime::parse_from_str(&maghrib, "%a %d %b %Y, %k:%M")?;
        let isha = NaiveDateTime::parse_from_str(&isha, "%a %d %b %Y, %k:%M")?;

        let rhs = Duration::hours(12);
        Ok(Day {
            date: fajr.date(),
            prayers: [
                Prayer::new(Kind::Fajr, fajr.time()),
                Prayer::new(Kind::Dhuhr, dhuhr.time().overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Asr, asr.time().overflowing_add_signed(rhs).0)
                    .apply_asr_correction(),
                Prayer::new(Kind::Maghrib, maghrib.time().overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Isha, isha.time().overflowing_add_signed(rhs).0),
            ],
        })
    }
}

pub async fn from_csv() -> Result<Vec<Day>, Box<dyn std::error::Error>> {
    let data = download_csv_file().await?;

    let mut csv_reader = csv::Reader::from_reader(data.as_bytes());

    let mut days = vec![];
    for record in csv_reader.records() {
        let day = record
            .and_then(|x| x.deserialize::<'_, CSVPrayer>(None))?
            .build()?;
        days.push(day);
    }

    let mut this_month = std::fs::File::create("this_month.yaml").expect("Creating new file");

    serde_yaml::to_writer(&mut this_month, &days).expect("Writing to file");

    Ok(days)
}

async fn download_csv_file() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(DATA).await?;
    let content = response.text().await?;
    Ok(content)
}
