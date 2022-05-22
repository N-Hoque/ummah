//! Module for holding a [Month] of [Prayers](super::prayer::Prayer)

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::day::Day;

/// Tuple struct containing all the [Days](Day) of that month
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Month(Vec<Day>);

impl Month {
    pub fn new(days: Vec<Day>) -> Self {
        Self(days)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Day> {
        self.0.iter()
    }

    pub fn update_day(&mut self, day: &Day) {
        for current_day in self.0.iter_mut() {
            if day.get_date() == current_day.get_date() {
                *current_day = day.clone();
            }
        }
    }

    pub fn today(&self) -> Option<&Day> {
        self.0
            .iter()
            .find(|d| d.get_date() == chrono::Local::today().naive_utc())
    }

    pub fn today_mut(&mut self) -> Option<&mut Day> {
        self.0
            .iter_mut()
            .find(|d| d.get_date() == chrono::Local::today().naive_utc())
    }

    pub fn tomorrow(&self) -> Option<&Day> {
        self.0.iter().find(|d| {
            d.get_date() == chrono::Local::today().naive_utc() + chrono::Duration::days(1)
        })
    }

    pub fn select_by_day_offset(&self, days_offset: i64) -> Option<&Day> {
        self.0.iter().find(|d| {
            d.get_date() == chrono::Local::today().naive_utc() + chrono::Duration::days(days_offset)
        })
    }

    pub fn select_by_date(&self, date: NaiveDate) -> Option<&Day> {
        self.0.iter().find(|d| d.get_date() == date)
    }
}
