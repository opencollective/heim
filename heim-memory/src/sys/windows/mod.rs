/// https://docs.microsoft.com/en-US/windows/desktop/api/sysinfoapi/ns-sysinfoapi-_memorystatusex

use std::fmt;
use std::mem;

use winapi::shared::minwindef;
use winapi::um::sysinfoapi;

use heim_common::prelude::*;
use heim_common::units::{Information, information};

#[derive(Clone)]
pub struct Memory(sysinfoapi::MEMORYSTATUSEX);

impl Memory {
    pub fn total(&self) -> Information {
        Information::new::<information::byte>(self.0.ullTotalPhys)
    }

    pub fn available(&self) -> Information {
        Information::new::<information::byte>(self.0.ullAvailPhys)
    }

    pub fn free(&self) -> Information {
        self.available()
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("total", &self.total())
            .field("available", &self.available())
            .field("free", &self.free())
            .finish()
    }
}

#[derive(Clone)]
pub struct Swap(sysinfoapi::MEMORYSTATUSEX);

impl Swap {
    pub fn total(&self) -> Information {
        Information::new::<information::byte>(self.0.ullTotalPageFile)
    }

    pub fn used(&self) -> Information {
        self.total() - self.free()
    }

    pub fn free(&self) -> Information {
        Information::new::<information::byte>(self.0.ullAvailPageFile)
    }

}

impl fmt::Debug for Swap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Swap")
            .field("total", &self.total())
            .field("used", &self.used())
            .field("free", &self.free())
            .finish()
    }
}


fn memory_status() -> impl Future<Output=Result<sysinfoapi::MEMORYSTATUSEX>> {
    future::lazy(|_| {
        unsafe {
            let mut mem_status = mem::MaybeUninit::<sysinfoapi::MEMORYSTATUSEX>::uninit();
            let length = mem::size_of::<sysinfoapi::MEMORYSTATUSEX>() as minwindef::DWORD;
            (*mem_status.as_mut_ptr()).dwLength = length;

            let result = sysinfoapi::GlobalMemoryStatusEx(mem_status.as_mut_ptr());
            if result == 0 {
                Err(Error::last_os_error())
            } else {
                Ok(mem_status.assume_init())
            }
        }
    })
}

pub fn swap() -> impl Future<Output=Result<Swap>> {
    memory_status()
        .map_ok(Swap)
}


pub fn memory() -> impl Future<Output=Result<Memory>> {
    memory_status()
        .map_ok(Memory)
}
