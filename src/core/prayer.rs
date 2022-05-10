//! Module for holding [Prayer] struct

use crate::types::PrayerName;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use std::fmt;

/// Represents an individual prayer
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    name: PrayerName,
    time: NaiveTime,
    performed: bool,
}

impl fmt::Display for Prayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            PrayerName::Fajr => write!(f, "Fajr: {}", self.time),
            PrayerName::Dhuhr => write!(f, "Dhuhr: {}", self.time),
            PrayerName::Asr => write!(f, "Asr: {}", self.time),
            PrayerName::Maghrib => write!(f, "Maghrib: {}", self.time),
            PrayerName::Isha => write!(f, "Isha: {}", self.time),
        }
    }
}

impl Prayer {
    pub(crate) fn new(name: PrayerName, time: NaiveTime) -> Self {
        Self {
            name,
            time,
            performed: chrono::Local::now().time() >= time,
        }
    }

    /// Checks if the prayer has been performed
    pub fn is_performed(&self) -> bool {
        self.performed
    }

    /// Set the prayer as performed
    pub fn set_performed(&mut self) {
        self.performed = true;
    }

    /// Gets the prayer name
    pub fn get_name(&self) -> PrayerName {
        self.name
    }

    /// Gets the prayer time
    pub fn get_time(&self) -> NaiveTime {
        self.time
    }
}
