use std::mem::zeroed;

use libc::{settimeofday, timeval, timezone};
use nats;

const USEC_PER_SEC: u128 = 1_000_000u128;

fn set_system_time(epoch_us: u128) -> Result<(), std::io::Error> {
    let mut tv: timeval = unsafe { zeroed() };
    tv.tv_sec = (epoch_us / USEC_PER_SEC) as i64;
    tv.tv_usec = (epoch_us % USEC_PER_SEC) as i64;

    unsafe {
        // settimeofday no longer takes a timezone as a second parameter.
        // Passing anything other than null will result in a failure.
        let null_tz: *const timezone = std::ptr::null();
        if settimeofday(&tv, null_tz) != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nats_url = "localhost";
    let time_subject = "time.system";

    // The NATS server may not be fully initialized when this process starts.
    // Ensure that we retry on failed connections, using the default nats.rs backoff.
    let nats_options = nats::Options::new().retry_on_failed_connect();
    println!("Connecting to NATS URL {}", nats_url);
    let nats_client = nats_options.connect(nats_url)?;

    println!("Subscribing to subject '{}'", time_subject);
    let time_sub = nats_client.subscribe(time_subject)?;

    for msg in time_sub.messages() {
        if msg.data.len() != std::mem::size_of::<u128>() {
            println!(
                "Skipping {}-byte message; expected {} bytes",
                msg.data.len(),
                std::mem::size_of::<u128>()
            );
            continue;
        }

        let epoch_us = u128::from_le_bytes(msg.data.try_into().unwrap());

        let local_before = chrono::Local::now();
        set_system_time(epoch_us)?;
        let local_after = chrono::Local::now();

        println!(
            "Received timestamp {}; local time went from {} to {}",
            epoch_us, local_before, local_after
        );
    }

    Ok(())
}
