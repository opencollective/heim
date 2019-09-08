#![no_main]
#![cfg(target_os = "linux")]

#[macro_use]
extern crate libfuzzer_sys;

use std::str::{self, FromStr};

use heim_memory::sys::{Swap, VmStat};
extern crate heim_memory;

lazy_static::lazy_static! {
    static ref VM_STAT: VmStat = VmStat::from_str(include_str!("../../assets/proc_vmstat.txt")).unwrap();
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let _ = Swap::parse_str(s, VM_STAT.clone());
    }
});
