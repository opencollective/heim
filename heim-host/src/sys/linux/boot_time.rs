use std::io;

use heim_common::prelude::*;
use heim_common::units::{Time, time};
use heim_runtime::fs;

pub async fn boot_time() -> Result<Time> {
    let mut lines = fs::read_lines("/proc/stat");
    while let Some(line) = lines.next().await {
        let line = line?;
        if !line.starts_with("btime ") {
            continue;
        }

        return match line.splitn(2, ' ').nth(1) {
            Some(raw_value) => {
                raw_value.parse::<f64>()
                    .map(Time::new::<time::second>)
                    .map_err(Into::into)
            },
            // TODO: Add details on why data is wrong
            None => Err(io::Error::from(io::ErrorKind::InvalidData).into())
        }
    }

    // TODO: Add details on why data is wrong
    Err(io::Error::from(io::ErrorKind::InvalidData).into())
}
