use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_unix_timestamp_as_secs() -> u64 {
    let ns = SystemTime::now().duration_since(UNIX_EPOCH);
    if ns.is_ok() {
        return ns.unwrap().as_secs();
    }
    return 0;
}
pub fn get_current_unix_timestamp_as_milsecs() -> u128 {
    let ns = SystemTime::now().duration_since(UNIX_EPOCH);
    if ns.is_ok() {
        return ns.unwrap().as_millis();
    }
    return 0;
}
