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
    #[serde(skip)]
    performed: bool,
}

impl PartialOrd for Prayer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => self.time.partial_cmp(&other.time),
            ord => ord,
        }
    }
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
    /// Checks if the prayer has been performed
    #[must_use]
    pub const fn is_performed(&self) -> bool {
        self.performed
    }

    /// Set the prayer as performed
    pub fn set_performed(&mut self, is_performed: bool) {
        self.performed = is_performed;
    }

    /// Gets the prayer name
    #[must_use]
    pub const fn get_name(&self) -> PrayerName {
        self.name
    }

    /// Gets the prayer time
    #[must_use]
    pub const fn get_time(&self) -> NaiveTime {
        self.time
    }

    #[must_use]
    pub(crate) const fn new(name: PrayerName, time: NaiveTime, performed: bool) -> Self {
        Self {
            name,
            time,
            performed,
        }
    }
}
