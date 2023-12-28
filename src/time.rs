use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> f64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as f64
        / 1e9
}

