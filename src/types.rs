use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::{error, fmt, io};

/// Wrapper around [Result]
pub type UmmahResult<T> = Result<T, UmmahError>;

/// Names for all the prayers
///
/// TODO: Add support for Taraweeh and Tahajjud.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PrayerName {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl fmt::Display for PrayerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrayerName::Fajr => write!(f, "Fajr"),
            PrayerName::Dhuhr => write!(f, "Dhuhr"),
            PrayerName::Asr => write!(f, "Asr"),
            PrayerName::Maghrib => write!(f, "Maghrib"),
            PrayerName::Isha => write!(f, "Isha"),
        }
    }
}

/// The method to determine the height of the sun
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub enum LatitudeMethod {
    OneSeventh = 3,
    AngleBased,
}

/// The organisation to base the calculations from
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub enum PrayerMethod {
    /// Muslim World League
    MWL = 1,
    /// University of Islamic Sciences
    UIS = 3,
    /// Islamic Society of North America
    ISNA = 5,
}

/// The school of thought to follow for the afternoon prayer
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub enum AsrMethod {
    Shafi = 1,
    Hanafi,
}

/// Represents all possible program errors
#[derive(Debug, Error)]
pub enum UmmahError {
    /// Thrown when obtaining prayer time
    #[error("Cannot get new time")]
    Prayer,

    /// Thrown when parsing CSV file
    #[error("Failed to read CSV file")]
    CSV(#[from] csv::Error),

    /// Thrown when parsing timestamps
    #[error("Failed to parse time")]
    DateTime(#[from] chrono::ParseError),

    /// Thrown on file/IO errors
    #[error("Failed to handle filesystem/IO")]
    IO(#[from] io::Error),

    /// Thrown on (de)serialization errors
    #[error("Failed to (de)serialize")]
    Serde(#[from] serde_yaml::Error),

    /// Thrown when attempting to submit request to website
    #[error("Failed to request times")]
    Unknown(#[from] Box<dyn error::Error>),
}
