//! Module for holding a [Day] of [Prayers](super::prayer::Prayer)

use crate::core::prayer::Prayer;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use std::{cmp::Ordering, fmt};

/// Holds all prayers for a given day
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Day {
    date: NaiveDate,
    prayers: [Prayer; 5],
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = self.date.format("%A, %d %B %Y").to_string();

        output = format!("\n{:^62}\n", output);

        output += &format!("|{:=<62}|\n|", "");

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.get_name().to_string());
            if idx < 4 {
                output += " | ";
            }
        }

        output += "|\n|";

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.get_time().to_string());
            if idx < 4 {
                output += " | ";
            }
        }

        output += &format!("|\n|{:=<62}|\n", "");

        write!(f, "{output}")
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Day {
    pub(crate) fn new(date: NaiveDate, prayers: [Prayer; 5]) -> Self {
        Self { date, prayers }
    }

    /// Gets the next unperformed prayer
    pub fn get_next_prayer(&self) -> Option<&Prayer> {
        self.prayers.iter().find(|prayer| !prayer.is_performed())
    }

    /// Mutably gets the next unperformed prayer
    pub fn get_next_prayer_mut(&mut self) -> Option<&mut Prayer> {
        self.prayers
            .iter_mut()
            .find(|prayer| !prayer.is_performed())
    }

    /// Gets the date for the day
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    /// Gets all prayers for the day
    pub fn get_prayers(&self) -> [Prayer; 5] {
        self.prayers
    }
}
