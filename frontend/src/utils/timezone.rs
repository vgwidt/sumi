use chrono::{NaiveDateTime, TimeZone};

pub fn display_time(time: &NaiveDateTime, timezone: &Option<String>, format: &str) -> String {
    let tz = match timezone {
        Some(user_tz) => user_tz.parse().unwrap(),
        None => chrono_tz::Tz::UTC,
    };

    log::info!("Timezone: {}", tz.to_string());

    let tz_aware = tz.from_utc_datetime(time);

    tz_aware.format(format).to_string()
    
}