use crate::types::{AsrCalculationMethod, LatitudeMethod, PrayerCalculationMethod};

use chrono::{Datelike, NaiveDate};
use chrono_utilities::naive::DateTransitions;

static LINK: &str = "https://www.salahtimes.com/uk/bath/csv";

pub struct PrayerQueryBuilder {
    pub(crate) high_latitude_method: LatitudeMethod,
    pub(crate) prayer_calculation_method: PrayerCalculationMethod,
    pub(crate) asr_calculation_method: AsrCalculationMethod,
    pub(crate) current_month: NaiveDate,
}

impl PrayerQueryBuilder {
    pub(crate) fn build(self) -> String {
        let current_year = self.current_month.year();
        let current_month = self.current_month.month();
        let end_day = self.current_month.last_day_of_month();

        let start_date = format!("{}-{}-01", current_year, current_month);
        let end_date = format!("{}-{}-{}", current_year, current_month, end_day);

        format!(
        "{}?highlatitudemethod={}&prayercalculationmethod={}&asarcalculationmethod={}&start={}&end={}",
        LINK,
        self.high_latitude_method as u8,
        self.prayer_calculation_method as u8,
        self.asr_calculation_method as u8,
        start_date,
        end_date
    )
    }
}
