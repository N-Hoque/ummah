//! Module for holding a [Day] of [Prayers](super::prayer::Prayer)

use crate::core::{get_performed_status, prayer::Prayer};

use chrono::NaiveDate;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};

use std::{cmp::Ordering, fmt};

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum DayField {
    Date,
    Prayers,
}

struct DayVisitor;

impl<'de> Visitor<'de> for DayVisitor {
    type Value = Day;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Day")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Day, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let date = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let prayers = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        Ok(Day::new(date, prayers))
    }

    fn visit_map<V>(self, mut map: V) -> Result<Day, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut date = None;
        let mut prayers = None;
        while let Some(key) = map.next_key()? {
            match key {
                DayField::Date => {
                    if date.is_some() {
                        return Err(serde::de::Error::duplicate_field("date"));
                    }
                    date = Some(map.next_value()?);
                }
                DayField::Prayers => {
                    if prayers.is_some() {
                        return Err(serde::de::Error::duplicate_field("prayers"));
                    }
                    prayers = Some(map.next_value()?);
                }
            }
        }
        let date = date.ok_or_else(|| serde::de::Error::missing_field("date"))?;
        let mut prayers: [Prayer; 5] =
            prayers.ok_or_else(|| serde::de::Error::missing_field("prayers"))?;

        for prayer in prayers.iter_mut() {
            prayer.set_performed(get_performed_status(date, prayer.get_time()));
        }

        Ok(Day::new(date, prayers))
    }
}

/// Holds all prayers for a given day
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Day {
    date: NaiveDate,
    prayers: [Prayer; 5],
}

impl<'de> Deserialize<'de> for Day {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Prayer", &["date", "prayers"], DayVisitor)
    }
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

    pub(crate) fn new(date: NaiveDate, prayers: [Prayer; 5]) -> Self {
        Self { date, prayers }
    }
}
