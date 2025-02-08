use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;
pub fn current_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seconds = now.as_secs();
    let tm = OffsetDateTime::from_unix_timestamp(seconds as i64).expect("Invalid timestamp");

    format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        tm.year(),
        tm.month() as u8,
        tm.day(),
        tm.hour(),
        tm.minute(),
        tm.second()
    )
}
