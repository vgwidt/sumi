use chrono::{NaiveDateTime, FixedOffset, TimeZone};

pub fn to_local_time(time: &NaiveDateTime, offset: i32) -> NaiveDateTime {
    let local: FixedOffset = FixedOffset::east_opt(offset * 60).unwrap();

    local.from_utc_datetime(&time).naive_local()
}