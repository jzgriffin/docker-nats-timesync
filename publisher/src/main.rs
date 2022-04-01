use std::time::Duration;

use chrono::{self, TimeZone, DateTime};
use nats;

const USEC_PER_SEC: u128 = 1_000_000u128;

fn datetime_microseconds<Tz: TimeZone>(dt: DateTime<Tz>) -> u128 {
    u128::try_from(dt.timestamp()).unwrap() * USEC_PER_SEC
        + u128::from(dt.timestamp_subsec_micros())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nats_url = "localhost";
    let time_subject = "time.system";
    let publish_interval = Duration::from_millis(500);

    // The NATS server may not be fully initialized when this process starts.
    // Ensure that we retry on failed connections, using the default nats.rs backoff.
    let nats_options = nats::Options::new().retry_on_failed_connect();
    println!("Connecting to NATS URL {}", nats_url);
    let nats_client = nats_options.connect(nats_url)?;

    loop {
        // Publish the time as local microseconds since the epoch.
        let now_us = datetime_microseconds(chrono::Local::now());
        println!("Sending timestamp {}", now_us);
        nats_client.publish(time_subject, now_us.to_le_bytes())?;

        std::thread::sleep(publish_interval);
    }
}
