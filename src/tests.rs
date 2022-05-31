#[cfg(test)]
mod performed_status {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use crate::get_performed_status;

    fn get_test_date() -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd(2022, 1, 15),
            NaiveTime::from_hms(18, 0, 0),
        )
    }

    #[test]
    fn before_current_date_marks_prayer_as_performed() {
        let prayer_date = NaiveDate::from_ymd(2022, 1, 1);
        let prayer_time = NaiveTime::from_hms(0, 0, 0);

        assert!(get_performed_status(
            get_test_date(),
            prayer_date,
            prayer_time
        ))
    }

    #[test]
    fn on_current_date_and_prayer_time_before_current_time_marks_prayer_as_performed() {
        let prayer_date = NaiveDate::from_ymd(2022, 1, 15);
        let prayer_time = NaiveTime::from_hms(0, 0, 0);

        assert!(get_performed_status(
            get_test_date(),
            prayer_date,
            prayer_time
        ))
    }

    #[test]
    fn on_current_date_and_prayer_time_on_current_time_marks_prayer_as_performed() {
        let prayer_date = NaiveDate::from_ymd(2022, 1, 15);
        let prayer_time = NaiveTime::from_hms(18, 0, 0);

        assert!(get_performed_status(
            get_test_date(),
            prayer_date,
            prayer_time
        ))
    }

    #[test]
    fn on_current_date_but_prayer_time_after_current_time_does_not_mark_prayer_as_performed() {
        let prayer_date = NaiveDate::from_ymd(2022, 1, 15);
        let prayer_time = NaiveTime::from_hms(18, 0, 1);

        assert!(!get_performed_status(
            get_test_date(),
            prayer_date,
            prayer_time
        ))
    }

    #[test]
    fn after_current_date_does_not_mark_prayer_as_performed() {
        let prayer_date = NaiveDate::from_ymd(2022, 1, 16);
        let prayer_time = NaiveTime::from_hms(0, 0, 0);

        assert!(!get_performed_status(
            get_test_date(),
            prayer_date,
            prayer_time
        ))
    }
}
