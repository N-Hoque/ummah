use crate::types::Kind;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    pub(crate) kind: Kind,
    pub(crate) time: NaiveTime,
}

impl fmt::Display for Prayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::Fajr => write!(f, "Fajr: {}", self.time),
            Kind::Dhuhr => write!(f, "Dhuhr: {}", self.time),
            Kind::Asr => write!(f, "Asr: {}", self.time),
            Kind::Maghrib => write!(f, "Maghrib: {}", self.time),
            Kind::Isha => write!(f, "Isha: {}", self.time),
        }
    }
}

impl Prayer {
    pub(crate) fn new(kind: Kind, time: NaiveTime) -> Self {
        Self { kind, time }
    }
}
