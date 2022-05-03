use crate::prayer::Prayer;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use std::{cmp::Ordering, fmt};

/// Holds all prayers for a given day
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Day {
    pub(crate) date: NaiveDate,
    pub(crate) prayers: [Prayer; 5],
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = self.date.format("%A, %d %B %Y").to_string();

        output = format!("\n{:^62}\n", output);

        output += &format!("|{:=<62}|\n|", "");

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.kind.to_string());
            if idx < 4 {
                output += " | ";
            }
        }

        output += "|\n|";

        for (idx, prayer) in self.prayers.iter().enumerate() {
            output += &format!("{:^10}", prayer.time.to_string());
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
    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn get_prayers(&self) -> [Prayer; 5] {
        self.prayers
    }
}
