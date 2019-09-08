use std::io;

use heim_common::prelude::*;
use heim_common::units::{Time, time};
use heim_runtime::fs;

#[allow(clippy::redundant_closure)]
pub async fn uptime() -> Result<Time> {
    let contents = fs::read_to_string("/proc/uptime").await?;

    match contents.splitn(2, ' ').next() {
        Some(raw_value) => {
            let seconds = raw_value.parse::<f64>()?;

            Ok(Time::new::<time::second>(seconds))
        },
        None => Err(io::Error::from(io::ErrorKind::InvalidData).into())
    }
}
