use futures_util::lock::Mutex;

use heim_common::prelude::*;

use crate::{sys, Time};

lazy_static::lazy_static! {
    static ref BOOT_TIME: Mutex<Option<Time>> = Mutex::default();
}

/// Returns system boot [Time] since the UNIX epoch.
///
/// Successfully resolved value is cached internally,
/// so all consequent calls will re-use the same value.
///
/// [Time]: ./struct.Time.html
pub async fn boot_time() -> Result<Time> {
    // TODO: Mutex here is an obviously bad decision,
    // but at this moment there is no async RwLock exists,
    // which can comply to this requirements:
    //   1. futures 0.3 aware
    //   2. not to be bound to some specific runtime (e.g. `async-std` RwLock)
    //
    // If there will be any, it would be nice to use it,
    // as it would lead to cheaper read operations
    let mut guard = BOOT_TIME.lock().await;

    match *guard {
        Some(time) => Ok(time),
        None => {
            let time = sys::boot_time().await?;
            *guard = Some(time);
            Ok(time)
        }
    }
}
