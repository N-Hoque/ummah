use crate::{
    day::Day,
    prayer::Prayer,
    types::{
        AdhanError, AdhanResult, AsrCalculationMethod, Kind, LatitudeMethod,
        PrayerCalculationMethod,
    },
};

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use chrono_utilities::naive::DateTransitions;
use serde::Deserialize;

static LINK: &str = "https://www.salahtimes.com/uk/bath/csv";

pub struct PrayerQueryBuilder {
    pub(crate) high_latitude_method: LatitudeMethod,
    pub(crate) prayer_calculation_method: PrayerCalculationMethod,
    pub(crate) asr_calculation_method: AsrCalculationMethod,
    pub(crate) current_month: NaiveDate,
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
    _day: String,
    fajr: String,
    _s: String,
    dhuhr: String,
    asr: String,
    maghrib: String,
    isha: String,
}

impl CSVPrayer {
    pub fn build(self) -> AdhanResult<Day> {
        let fajr = parse_prayer_time(&self.fajr)?;
        let dhuhr = parse_prayer_time(&self.dhuhr)?;
        let asr = parse_prayer_time(&self.asr)?;
        let maghrib = parse_prayer_time(&self.maghrib)?;
        let isha = parse_prayer_time(&self.isha)?;

        let rhs = Duration::hours(12);
        Ok(Day {
            date: Local::now().date().naive_utc(),
            prayers: [
                Prayer::new(Kind::Fajr, fajr),
                Prayer::new(Kind::Dhuhr, dhuhr.overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Asr, asr.overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Maghrib, maghrib.overflowing_add_signed(rhs).0),
                Prayer::new(Kind::Isha, isha.overflowing_add_signed(rhs).0),
            ],
        })
    }
}

static TIME_FMT: &str = "%k:%M";

fn parse_prayer_time(prayer_time: &str) -> AdhanResult<NaiveTime> {
    NaiveTime::parse_from_str(prayer_time, TIME_FMT).map_err(AdhanError::DateTime)
}
