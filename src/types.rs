use clap::ArgEnum;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdhanError {
    #[error("Failed to request times")]
    Request,

    #[error("Failed to download times")]
    Download,

    #[error("Failed to deserialize times")]
    Deserialize,

    #[error("Failed to parse time")]
    Parse,

    #[error("Failed to create file")]
    FileCreation(#[from] std::io::Error),

    #[error("Failed to serialize and write file")]
    SerializedFileWrite(#[from] serde_yaml::Error),
}

#[derive(Debug, Clone, Copy, ArgEnum)]
pub(crate) enum LatitudeMethod {
    OneSeventh = 3,
    AngleBased,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, ArgEnum)]
pub(crate) enum PrayerCalculationMethod {
    MWL = 1,
    UIS = 3,
    ISNA = 5,
}

#[derive(Debug, Clone, Copy, ArgEnum)]
pub(crate) enum AsrCalculationMethod {
    Shafi = 1,
    Hanafi,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}
