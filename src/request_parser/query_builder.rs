use crate::types::{AsrMethod, LatitudeMethod, PrayerMethod};

use chrono::{Datelike, NaiveDate};
use chrono_utilities::naive::DateTransitions;

static LINK: &str = "https://www.salahtimes.com/";

pub struct PrayerQueryBuilder {
    pub(crate) high_latitude_method: LatitudeMethod,
    pub(crate) prayer_calculation_method: PrayerMethod,
    pub(crate) asr_calculation_method: AsrMethod,
    pub(crate) current_month: NaiveDate,
}

impl PrayerQueryBuilder {
    pub(crate) fn build(self, country: &str, city: &str) -> String {
        let current_year = self.current_month.year();
        let current_month = self.current_month.month();
        let end_day = self.current_month.last_day_of_month();

        let start_date = format!("{}-{}-01", current_year, current_month);
        let end_date = format!("{}-{}-{}", current_year, current_month, end_day);

        format!(
        "{}/{}/{}/csv?highlatitudemethod={}&prayercalculationmethod={}&asarcalculationmethod={}&start={}&end={}",
        LINK, country, city,
        self.high_latitude_method as u8,
        self.prayer_calculation_method as u8,
        self.asr_calculation_method as u8,
        start_date,
        end_date
    )
    }
}
