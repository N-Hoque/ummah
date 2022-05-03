use crate::{
    request_parser::query_builder::PrayerQueryBuilder,
    types::{AsrCalculationMethod, LatitudeMethod, PrayerCalculationMethod},
};

use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

/// Settings for calculating prayer times and determining current month of prayers
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayerSettings {
    pub(crate) latitude_method: LatitudeMethod,
    pub(crate) prayer_method: PrayerCalculationMethod,
    pub(crate) asr_method: AsrCalculationMethod,
    current_month: u32,
}

impl PrayerSettings {
    pub fn new(
        latitude_method: LatitudeMethod,
        prayer_method: PrayerCalculationMethod,
        asr_method: AsrCalculationMethod,
    ) -> Self {
        Self {
            latitude_method,
            prayer_method,
            asr_method,
            current_month: Local::now().month(),
        }
    }

    pub(crate) fn query(&self) -> String {
        PrayerQueryBuilder {
            high_latitude_method: self.latitude_method,
            prayer_calculation_method: self.prayer_method,
            asr_calculation_method: self.asr_method,
            current_month: Local::now().naive_utc().date(),
        }
        .build()
    }
}
