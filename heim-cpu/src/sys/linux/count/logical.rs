use std::io;

use heim_common::prelude::*;
use heim_runtime::fs;

fn sysconf() -> io::Result<u64> {
    let result = unsafe {
        libc::sysconf(libc::_SC_NPROCESSORS_ONLN)
    };

    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result as u64)
    }
}

async fn cpuinfo() -> Result<u64> {
    let mut amount = 0;
    let mut lines = fs::read_lines("/proc/cpuinfo");
    while let Some(line) = lines.next().await {
        if line?.starts_with("processor") {
            amount += 1;
        }
    }

    Ok(amount)
}

pub async fn logical_count() -> Result<u64> {
    match sysconf() {
        Ok(value) => Ok(value),
        Err(..) => cpuinfo().await
    }

    // TODO: Parse the `/proc/stat` to support old systems
    // See https://github.com/giampaolo/psutil/issues/200
}
