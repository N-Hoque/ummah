pub mod settings;

use crate::types::PrayerName;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use std::fmt;

/// Represents an individual prayer
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    pub(crate) kind: PrayerName,
    pub(crate) time: NaiveTime,
}

impl fmt::Display for Prayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            PrayerName::Fajr => write!(f, "Fajr: {}", self.time),
            PrayerName::Dhuhr => write!(f, "Dhuhr: {}", self.time),
            PrayerName::Asr => write!(f, "Asr: {}", self.time),
            PrayerName::Maghrib => write!(f, "Maghrib: {}", self.time),
            PrayerName::Isha => write!(f, "Isha: {}", self.time),
        }
    }
}

impl Prayer {
    pub(crate) fn new(kind: PrayerName, time: NaiveTime) -> Self {
        Self { kind, time }
    }
}
