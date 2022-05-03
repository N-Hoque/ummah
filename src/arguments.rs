use crate::{
    prayer::settings::PrayerSettings,
    types::{AsrCalculationMethod, LatitudeMethod, PrayerCalculationMethod},
};

use clap::Parser;
use serde::{Deserialize, Serialize};

/// Gets prayer times from www.salahtimes.com/uk
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct PrayerArguments {
    /// Latitude method
    #[clap(short, long, arg_enum, default_value = "one-seventh")]
    latitude_method: LatitudeMethod,

    /// Source of Prayer calculation
    #[clap(short, long, arg_enum, default_value = "mwl")]
    prayer_method: PrayerCalculationMethod,

    /// Asr time method
    #[clap(short, long, arg_enum, default_value = "shafi")]
    asr_method: AsrCalculationMethod,

    /// Get today's times
    #[clap(short, long)]
    today_only: bool,
}

impl PrayerArguments {
    /// Get prayer calculation settings
    pub fn settings(&self) -> PrayerSettings {
        PrayerSettings::new(self.latitude_method, self.prayer_method, self.asr_method)
    }

    /// Flag for selecting only today's prayer times
    pub fn is_today_only(&self) -> bool {
        self.today_only
    }
}
