use std::io;
use std::mem;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::{sysinfoapi, winnt, winbase};
use winapi::shared::{ntstatus, minwindef};
use ntapi::ntrtl;

use super::bindings::MAX_COMPUTERNAME_LENGTH;

mod sessions;
mod session;
mod wts_info;

pub use self::sessions::Sessions;
pub use self::session::Session;
pub use self::wts_info::WtsInfo;

// Partial copy of the `sysinfoapi::SYSTEM_INFO`,
// because it contains pointers and we need to sent it between threads.
// TODO: It would be better to make `SYSTEM_INFO` Sendable somehow?
#[derive(Debug)]
pub struct SystemInfo {
    pub processor_arch: minwindef::WORD,
}

impl From<sysinfoapi::SYSTEM_INFO> for SystemInfo {
    fn from(info: sysinfoapi::SYSTEM_INFO) -> SystemInfo {
        let s = unsafe {
            info.u.s()
        };

        SystemInfo {
            processor_arch: s.wProcessorArchitecture,
        }
    }
}

pub fn get_native_system_info() -> SystemInfo {
    let mut info = mem::MaybeUninit::<sysinfoapi::SYSTEM_INFO>::uninit();

    let info = unsafe {
        // `GetNativeSystemInfo` returns `void`
        // and it seems that it can't fail
        // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getnativesysteminfo
        sysinfoapi::GetNativeSystemInfo(info.as_mut_ptr());

        info.assume_init()
    };

    info.into()
}

pub fn rtl_get_version() -> io::Result<winnt::OSVERSIONINFOEXW> {
    let mut osinfo = mem::MaybeUninit::<winnt::RTL_OSVERSIONINFOEXW>::uninit();

    let result = unsafe {
        (*osinfo.as_mut_ptr()).dwOSVersionInfoSize = mem::size_of::<winnt::RTL_OSVERSIONINFOEXW>() as minwindef::DWORD;

        // https://docs.microsoft.com/en-us/windows/win32/devnotes/rtlgetversion
        // Also, tricking `RtlGetVersion` to accept `*EX` struct by casting the pointer,
        // see the "Parameters" section from the link above
        ntrtl::RtlGetVersion(osinfo.as_mut_ptr() as winnt::PRTL_OSVERSIONINFOW)
    };

    if result == ntstatus::STATUS_SUCCESS {
        unsafe {
            Ok(osinfo.assume_init())
        }
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn get_computer_name() -> io::Result<String> {
    // The buffer size should be large enough to contain MAX_COMPUTERNAME_LENGTH + 1 characters.
    let mut buffer: Vec<winnt::WCHAR> = Vec::with_capacity((MAX_COMPUTERNAME_LENGTH + 1) as usize);
    let mut size: minwindef::DWORD = MAX_COMPUTERNAME_LENGTH + 1;

    let result = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getcomputernamew
        winbase::GetComputerNameW(buffer.as_mut_ptr(), &mut size)
    };
    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        unsafe {
            // `+ 1` is for terminating null character
            buffer.set_len(size as usize + 1);
        }
        let string = OsString::from_wide(&buffer[..(size as usize)]).to_string_lossy().to_string();

        Ok(string)
    }
}
