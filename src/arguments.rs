use crate::{
    prayer::settings::{CalculationMethods, Location, PrayerSettings},
    types::{AsrMethod, LatitudeMethod, PrayerMethod},
};

use clap::Parser;
use serde::{Deserialize, Serialize};

/// Gets prayer times from www.salahtimes.com/uk
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct PrayerArguments {
    /// Latitude method
    #[clap(long, arg_enum, default_value = "one-seventh")]
    latitude_method: LatitudeMethod,

    /// Source of Prayer calculation
    #[clap(long, arg_enum, default_value = "mwl")]
    prayer_method: PrayerMethod,

    /// Asr time method
    #[clap(long, arg_enum, default_value = "shafi")]
    asr_method: AsrMethod,

    /// Country
    #[clap(long, default_value = "uk")]
    country: String,

    /// City
    #[clap(long, default_value = "bath")]
    city: String,

    /// Get today's times
    #[clap(short, long)]
    today: bool,

    /// Exports times to HTML file
    #[clap(long)]
    export: bool,

    /// Generate default CSS for HTML file
    /// If not set, generates template CSS for custom editing
    #[clap(long)]
    generate_css: bool,

    /// Clear cache
    #[clap(long)]
    clear_cache: bool,
}

impl PrayerArguments {
    /// Get prayer calculation settings
    pub fn settings(&self) -> PrayerSettings {
        PrayerSettings::new(
            CalculationMethods {
                latitude: self.latitude_method,
                prayer: self.prayer_method,
                asr: self.asr_method,
            },
            Location {
                country: self.country.clone(),
                city: self.city.clone(),
            },
        )
    }

    pub fn clear_cache(&self) -> bool {
        self.clear_cache
    }

    /// Flag for selecting only today's prayer times
    pub fn is_today_only(&self) -> bool {
        self.today
    }

    pub fn export_enabled(&self) -> bool {
        self.export
    }

    pub fn generate_default_css(&self) -> bool {
        self.generate_css
    }
}
