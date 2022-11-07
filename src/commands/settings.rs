use crate::types::{AsrMethod, LatitudeMethod, Organisation};

use chrono::Datelike;
use chrono_utilities::naive::DateTransitions;
use serde::{Deserialize, Serialize};

static LINK: &str = "https://www.salahtimes.com/";

/// Settings for calculating prayer times and determining current month of prayers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrayerSettings {
    methods: CalculationMethods,
    location: Location,
    is_audio_downloaded: bool,
    current_month: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationMethods {
    pub latitude: LatitudeMethod,
    pub organisation: Organisation,
    pub asr: AsrMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub country: String,
    pub city: String,
}

impl PrayerSettings {
    #[must_use]
    pub fn new(methods: CalculationMethods, location: Location) -> Self {
        Self {
            methods,
            location,
            is_audio_downloaded: false,
            current_month: chrono::Local::now().month(),
        }
    }

    pub(crate) fn with_audio_downloaded(self) -> Self {
        Self {
            is_audio_downloaded: true,
            ..self
        }
    }

    /// Generates query out of settings
    pub(crate) fn query(&self, current_month: chrono::NaiveDate) -> String {
        let end_day = current_month.last_day_of_month();
        let current_year = current_month.year();
        let current_month = current_month.month();

        let start_date = format!("{}-{}-01", current_year, current_month);
        let end_date = format!("{}-{}-{}", current_year, current_month, end_day);

        format!(
            "{}/{}/{}/csv?highlatitudemethod={}&prayercalculationmethod={}&asarcalculationmethod={}&start={}&end={}",
            LINK, self.location.country, self.location.city,
            self.methods.latitude as u8,
            self.methods.organisation as u8,
            self.methods.asr as u8,
            start_date,
            end_date
        )
    }
}
