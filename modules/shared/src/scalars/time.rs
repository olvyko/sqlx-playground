pub use chrono::NaiveDateTime;
use chrono::{TimeZone, Utc};

pub fn now_unix_time() -> NaiveDateTime {
    now_unix_time_withtout_nanos()
}

fn now_unix_time_withtout_nanos() -> NaiveDateTime {
    let now_millis = Utc::now().timestamp_millis();
    Utc.timestamp_millis(now_millis).naive_utc()
}
