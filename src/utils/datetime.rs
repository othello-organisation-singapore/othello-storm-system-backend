use mocktopus::macros::mockable;

use std::time::{SystemTime, UNIX_EPOCH};

#[mockable]
pub fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}
