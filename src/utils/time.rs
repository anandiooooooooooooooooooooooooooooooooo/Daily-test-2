use chrono::{DateTime, Utc};

pub fn current_timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}
// Telemetry update recorded at Sun Jun 29 22:32:05 UTC 2025
// @1751239368 [77owefxR8BLR]
// @1751239674 [a7gAqI0SU02B]
