use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub fn display_time(time: &NaiveDateTime, timezone: &Option<String>, format: &str) -> String {
    if let Some(user_tz) = timezone {
        let tz: Tz = user_tz.parse().unwrap();
        let tz_aware = tz.from_utc_datetime(time);

        tz_aware.format(format).to_string()
    }
    else {
        time.format(format).to_string()
    }
    
}