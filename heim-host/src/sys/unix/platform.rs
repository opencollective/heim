use std::mem;
use std::str::FromStr;
use std::ffi::CStr;
use std::borrow::Cow;

use heim_common::prelude::*;

use crate::Arch;

#[derive(Debug)]
pub struct Platform {
    system: String,
    release: String,
    version: String,
    hostname: String,
    arch: Arch,
}

impl Platform {
    pub fn system(&self) -> &str {
        &self.system
    }

    pub fn release(&self) -> &str {
        &self.release
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn architecture(&self) -> Arch {
        self.arch
    }
}

#[inline]
fn cstr<'a>(ptr: *const libc::c_char) -> Cow<'a, str> {
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy()
    }
}

// Based on the https://github.com/uutils/platform-info/blob/master/src/unix.rs
pub async fn platform() -> Result<Platform> {
    let mut uts = mem::MaybeUninit::<libc::utsname>::uninit();
    let result = unsafe {
        libc::uname(uts.as_mut_ptr())
    };

    if result != 0 {
        Err(Error::last_os_error())
    } else {
        let uts = unsafe {
            uts.assume_init()
        };
        let raw_arch = cstr(uts.machine.as_ptr());
        let arch = Arch::from_str(&raw_arch).unwrap_or(Arch::Unknown);

        Ok(Platform {
            system: cstr(uts.sysname.as_ptr()).into_owned(),
            release: cstr(uts.release.as_ptr()).into_owned(),
            version: cstr(uts.version.as_ptr()).into_owned(),
            hostname: cstr(uts.nodename.as_ptr()).into_owned(),
            arch,
        })

    }
}
