use crate::{
    core::{get_performed_status, prayer::Prayer},
    time::{day::Day, month::Month},
    types::{PrayerName, UmmahError, UmmahResult},
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
        let rhs = Duration::hours(12);

        let date = parse_prayer_date(&self.day)?;

        let fajr = parse_prayer_time(&self.fajr, None)?;
        let dhuhr = parse_prayer_time(&self.dhuhr, Some(rhs))?;
        let asr = parse_prayer_time(&self.asr, Some(rhs))?;
        let maghrib = parse_prayer_time(&self.maghrib, Some(rhs))?;
        let isha = parse_prayer_time(&self.isha, Some(rhs))?;

        let day = Day::new(
            date,
            [
                Prayer::new(PrayerName::Fajr, fajr, get_performed_status(date, fajr)),
                Prayer::new(PrayerName::Dhuhr, dhuhr, get_performed_status(date, dhuhr)),
                Prayer::new(PrayerName::Asr, asr, get_performed_status(date, asr)),
                Prayer::new(
                    PrayerName::Maghrib,
                    maghrib,
                    get_performed_status(date, maghrib),
                ),
                Prayer::new(PrayerName::Isha, isha, get_performed_status(date, isha)),
            ],
        );

        Ok(day)
    }
}

fn parse_prayer_date(prayer_date: &str) -> UmmahResult<NaiveDate> {
    let prayer_date = format!("{} {}", prayer_date, Local::now().year());
    NaiveDate::parse_from_str(&prayer_date, DATE_FMT).map_err(UmmahError::DateTime)
}

fn parse_prayer_time(prayer_time: &str, with_retime: Option<Duration>) -> UmmahResult<NaiveTime> {
    let mut time =
        NaiveTime::parse_from_str(prayer_time, TIME_FMT).map_err(UmmahError::DateTime)?;

    if let Some(retime) = with_retime {
        time = time.overflowing_add_signed(retime).0;
    }

    Ok(time)
}
