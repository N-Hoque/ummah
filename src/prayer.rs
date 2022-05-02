use serde::{Deserialize, Serialize};

use crate::types::Kind;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    pub(crate) kind: Kind,
    pub(crate) time: chrono::NaiveTime,
}

impl std::fmt::Display for Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub(crate) fn new(kind: Kind, time: chrono::NaiveTime) -> Self {
        Self { kind, time }
    }
}
