use crate::types::{AsrMethod, LatitudeMethod, PrayerMethod};

use chrono::{Datelike, Local, NaiveDate};
use chrono_utilities::naive::DateTransitions;
use serde::{Deserialize, Serialize};

static LINK: &str = "https://www.salahtimes.com/";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CalculationMethods {
    pub(crate) latitude: LatitudeMethod,
    pub(crate) prayer: PrayerMethod,
    pub(crate) asr: AsrMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Location {
    pub(crate) country: String,
    pub(crate) city: String,
}

/// Settings for calculating prayer times and determining current month of prayers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayerSettings {
    pub(crate) methods: CalculationMethods,
    pub(crate) location: Location,
    is_audio_downloaded: bool,
    current_month: u32,
}

impl PrayerSettings {
    pub(crate) fn new(methods: CalculationMethods, location: Location) -> Self {
        Self {
            methods,
            location,
            is_audio_downloaded: false,
            current_month: Local::now().month(),
        }
    }

    pub(crate) fn with_audio_downloaded(self) -> Self {
        Self {
            is_audio_downloaded: true,
            ..self
        }
    }

    pub(crate) fn query(&self, current_month: NaiveDate) -> String {
        let end_day = current_month.last_day_of_month();
        let current_year = current_month.year();
        let current_month = current_month.month();

        let start_date = format!("{}-{}-01", current_year, current_month);
        let end_date = format!("{}-{}-{}", current_year, current_month, end_day);

        format!(
            "{}/{}/{}/csv?highlatitudemethod={}&prayercalculationmethod={}&asarcalculationmethod={}&start={}&end={}",
            LINK, self.location.country, self.location.city,
            self.methods.latitude as u8,
            self.methods.prayer as u8,
            self.methods.asr as u8,
            start_date,
            end_date
        )
    }
}
