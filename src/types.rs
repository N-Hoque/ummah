use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AdhanResult<T> = Result<T, AdhanError>;

#[derive(Debug, Error)]
pub enum AdhanError {
    #[error("Failed to request times")]
    Request,

    #[error("Failed to download times")]
    Download,

    #[error("Failed to read CSV file")]
    CSV(#[from] csv::Error),

    #[error("Failed to deserialize times")]
    Deserialize,

    #[error("Failed to parse time")]
    Parse,

    #[error("Failed to process file")]
    File(#[from] std::io::Error),

    #[error("Failed to (de)serialize")]
    Serde(#[from] serde_yaml::Error),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub(crate) enum LatitudeMethod {
    OneSeventh = 3,
    AngleBased,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub(crate) enum PrayerCalculationMethod {
    MWL = 1,
    UIS = 3,
    ISNA = 5,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, ArgEnum)]
pub(crate) enum AsrCalculationMethod {
    Shafi = 1,
    Hanafi,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
