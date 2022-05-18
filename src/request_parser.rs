use crate::{
    core::prayer::Prayer,
    time::{day::Day, month::Month},
    types::{UmmahError, UmmahResult, PrayerName},
};

use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use serde::Deserialize;

const MAX_DAYS: usize = 32;

static DATE_FMT: &str = "%a %d %b %Y";
static TIME_FMT: &str = "%k:%M";

pub fn parse_csv_file(data: bytes::Bytes) -> UmmahResult<Month> {
    let mut csv_reader = csv::Reader::from_reader(data.as_ref());
    let mut days = Vec::with_capacity(MAX_DAYS);
    for record in csv_reader.records() {
        let day = record
            .and_then(|x| x.deserialize::<'_, CSVPrayer>(None))
            .map_err(UmmahError::CSV)?
            .build()?;
        days.push(day);
    }
    Ok(Month::new(days))
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
    pub fn build(self) -> UmmahResult<Day> {
        let fajr = parse_prayer_time(&self.fajr)?;
        let dhuhr = parse_prayer_time(&self.dhuhr)?;
        let asr = parse_prayer_time(&self.asr)?;
        let maghrib = parse_prayer_time(&self.maghrib)?;
        let isha = parse_prayer_time(&self.isha)?;

        let rhs = Duration::hours(12);
        Ok(Day::new(
            parse_prayer_date(self.day)?,
            [
                Prayer::new(PrayerName::Fajr, fajr),
                Prayer::new(PrayerName::Dhuhr, dhuhr.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Asr, asr.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Maghrib, maghrib.overflowing_add_signed(rhs).0),
                Prayer::new(PrayerName::Isha, isha.overflowing_add_signed(rhs).0),
            ],
        ))
    }
}

fn parse_prayer_date(prayer_date: String) -> UmmahResult<NaiveDate> {
    let prayer_date = format!("{} {}", prayer_date, Local::now().year());
    NaiveDate::parse_from_str(&prayer_date, DATE_FMT).map_err(UmmahError::DateTime)
}

fn parse_prayer_time(prayer_time: &str) -> UmmahResult<NaiveTime> {
    NaiveTime::parse_from_str(prayer_time, TIME_FMT).map_err(UmmahError::DateTime)
}
