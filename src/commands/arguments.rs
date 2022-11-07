use crate::types::{AsrMethod, LatitudeMethod, Organisation};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::settings::{CalculationMethods, Location, PrayerSettings};

/// Gets prayer times from www.salahtimes.com/uk
#[derive(Parser, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct PrayerArguments {
    /// Latitude method
    #[clap(long, arg_enum, default_value = "one-seventh")]
    latitude_method: LatitudeMethod,

    /// Source of Prayer calculation
    #[clap(long, arg_enum, default_value = "mwl")]
    prayer_method: Organisation,

    /// Asr time method
    #[clap(long, arg_enum, default_value = "shafi")]
    asr_method: AsrMethod,

    /// The month to pull the timetable from
    #[clap(long)]
    month: Option<u32>,

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

    /// Specific output audio device
    #[clap(long)]
    output_device: String,
}

impl PrayerArguments {
    /// Get prayer calculation settings
    #[must_use]
    pub fn settings(&self) -> PrayerSettings {
        PrayerSettings::new(
            CalculationMethods {
                latitude: self.latitude_method,
                organisation: self.prayer_method,
                asr: self.asr_method,
            },
            Location {
                country: self.country.clone(),
                city: self.city.clone(),
            },
        )
    }

    /// Clears cache
    #[must_use]
    pub const fn clear_cache(&self) -> bool {
        self.clear_cache
    }

    /// Flag for selecting only today's prayer times
    #[must_use]
    pub const fn is_today_only(&self) -> bool {
        self.today
    }

    /// Flag for exporting timetable to HTML file
    #[must_use]
    pub const fn export_enabled(&self) -> bool {
        self.export
    }

    /// Flag for generating default CSS file for timetable
    #[must_use]
    pub const fn generate_default_css(&self) -> bool {
        self.generate_css
    }

    #[must_use]
    pub fn get_output_audio_device(&self) -> String {
        self.output_device.clone()
    }

    #[must_use]
    pub const fn month(&self) -> Option<u32> {
        self.month
    }
}
