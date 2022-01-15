use std::{thread, time::Duration};

pub async fn async_sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
