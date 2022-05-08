use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::day::Day;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Month(Vec<Day>);

impl Month {
    pub fn new(days: Vec<Day>) -> Self {
        Self(days)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Day> {
        self.0.iter()
    }

    pub fn today(&self) -> Option<&Day> {
        self.0
            .iter()
            .find(|d| d.date == chrono::Local::today().naive_utc())
    }

    pub fn tomorrow(&self) -> Option<&Day> {
        self.0
            .iter()
            .find(|d| d.date == chrono::Local::today().naive_utc() + chrono::Duration::days(1))
    }

    pub fn select_by_day_offset(&self, days_offset: i64) -> Option<&Day> {
        self.0.iter().find(|d| {
            d.date == chrono::Local::today().naive_utc() + chrono::Duration::days(days_offset)
        })
    }

    pub fn select_by_date(&self, date: NaiveDate) -> Option<&Day> {
        self.0.iter().find(|d| d.date == date)
    }
}
