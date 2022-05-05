use crate::{
    day::Day,
    prayer::Prayer,
    types::{AdhanError, AdhanResult, PrayerName},
};

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use serde::Deserialize;

pub fn parse_csv_file(data: bytes::Bytes) -> AdhanResult<Vec<Day>> {
    let mut csv_reader = csv::Reader::from_reader(data.as_ref());
    let mut days = vec![];
    for record in csv_reader.records() {
        let day = record
            .and_then(|x| x.deserialize::<'_, CSVPrayer>(None))
            .map_err(AdhanError::CSV)?
            .build()?;
        days.push(day);
    }
    Ok(days)
}

#[derive(Debug, Deserialize)]
pub(crate) struct CSVPrayer {
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
        let fajr = parse_prayer_time(&self.fajr)?;
        let dhuhr = parse_prayer_time(&self.dhuhr)?;
        let asr = parse_prayer_time(&self.asr)?;
        let maghrib = parse_prayer_time(&self.maghrib)?;
        let isha = parse_prayer_time(&self.isha)?;

        let rhs = Duration::hours(12);
        Ok(Day {
            date: parse_prayer_date(self.day)?,
            prayers: [
                Prayer::new(PrayerName::Fajr, fajr),
                Prayer::new(PrayerName::Dhuhr, dhuhr.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Asr, asr.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Maghrib, maghrib.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Isha, isha.overflowing_add_signed(rhs).0),
            ],
        })
    }
}

static DATE_FMT: &str = "%a %d %b %Y";
static TIME_FMT: &str = "%k:%M";

fn parse_prayer_date(prayer_date: String) -> AdhanResult<NaiveDate> {
    let prayer_date = format!("{} {}", prayer_date, Local::now().year());
    NaiveDate::parse_from_str(&prayer_date, DATE_FMT).map_err(AdhanError::DateTime)
}

fn parse_prayer_time(prayer_time: &str) -> AdhanResult<NaiveTime> {
    NaiveTime::parse_from_str(prayer_time, TIME_FMT).map_err(AdhanError::DateTime)
}
