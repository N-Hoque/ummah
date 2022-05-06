use crate::{
    request_parser::query_builder::PrayerQueryBuilder,
    types::{AsrMethod, LatitudeMethod, PrayerMethod},
};

use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CalculationMethods {
    pub(crate) latitude: LatitudeMethod,
    pub(crate) prayer: PrayerMethod,
    pub(crate) asr: AsrMethod,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Location {
    pub(crate) country: String,
    pub(crate) city: String,
}

/// Settings for calculating prayer times and determining current month of prayers
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayerSettings {
    pub(crate) methods: CalculationMethods,
    pub(crate) location: Location,
    current_month: u32,
}

impl PrayerSettings {
    pub(crate) fn new(methods: CalculationMethods, location: Location) -> Self {
        Self {
            methods,
            location,
            current_month: Local::now().month(),
        }
    }

    pub(crate) fn query(&self) -> String {
        PrayerQueryBuilder {
            high_latitude_method: self.methods.latitude,
            prayer_calculation_method: self.methods.prayer,
            asr_calculation_method: self.methods.asr,
            current_month: Local::now().naive_utc().date(),
        }
        .build(&self.location.country, &self.location.city)
    }
}
