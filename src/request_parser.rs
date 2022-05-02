use chrono::Datelike;
use chrono_utilities::naive::DateTransitions;
use serde::Deserialize;

use crate::{
    day::Day,
    prayer::Prayer,
    types::{
        AdhanError, AdhanResult, AsrCalculationMethod, Kind, LatitudeMethod,
        PrayerCalculationMethod,
    },
};

pub static LINK: &str = "https://www.salahtimes.com/uk/bath/csv";

pub struct PrayerQueryBuilder {
    pub(crate) high_latitude_method: LatitudeMethod,
    pub(crate) prayer_calculation_method: PrayerCalculationMethod,
    pub(crate) asr_calculation_method: AsrCalculationMethod,
    pub(crate) current_month: chrono::NaiveDate,
}

impl PrayerQueryBuilder {
    pub(crate) fn build(self) -> String {
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
pub struct CSVPrayer {
    day: String,
    fajr: String,
    _s: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

impl CSVPrayer {
    pub fn build(self) -> AdhanResult<Day> {
        let year = chrono::Utc::now().year();

        let fajr = format!("{} {}, {}", self.day, year, self.fajr);
        let dhuhr = format!("{} {}, {}", self.day, year, self.dhuhr);
        let asr = format!("{} {}, {}", self.day, year, self.asr);
        let maghrib = format!("{} {}, {}", self.day, year, self.maghrib);
        let isha = format!("{} {}, {}", self.day, year, self.isha);

        let fajr = chrono::NaiveDateTime::parse_from_str(&fajr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let dhuhr = chrono::NaiveDateTime::parse_from_str(&dhuhr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let asr = chrono::NaiveDateTime::parse_from_str(&asr, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let maghrib = chrono::NaiveDateTime::parse_from_str(&maghrib, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;
        let isha = chrono::NaiveDateTime::parse_from_str(&isha, "%a %d %b %Y, %k:%M")
            .map_err(|_| AdhanError::Parse)?;

        let rhs = chrono::Duration::hours(12);
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
