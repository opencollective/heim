#[macro_use]
extern crate criterion;

use std::str::FromStr;

use criterion::black_box;
use criterion::Criterion;

use heim_memory as memory;

static PROC_MEMINFO: &str = include_str!("../assets/proc_meminfo.txt");
static PROC_VMSTAT: &str = include_str!("../assets/proc_vmstat.txt");

fn bench_memory(c: &mut Criterion) {
    let mut executor = futures_executor::ThreadPool::new().unwrap();

    c.bench_function("memory", |b| {
        b.iter(|| executor.run(memory::memory()));
    });

    #[cfg(target_os = "linux")]
    c.bench_function("meminfo_parse", |b| {
        b.iter(|| memory::sys::Memory::from_str(black_box(&PROC_MEMINFO)));
    });
}

fn bench_swap(c: &mut Criterion) {
    let mut executor = futures_executor::ThreadPool::new().unwrap();

    c.bench_function("swap", |b| {
        b.iter(|| executor.run(memory::swap()));
    });

    #[cfg(target_os = "linux")]
    c.bench_function("vmstat_parse", |b| {
        b.iter(|| memory::sys::VmStat::from_str(black_box(&PROC_VMSTAT)));
    });
    #[cfg(target_os = "linux")]
    c.bench_function("swap_parse", |b| {
        let vm_stat = memory::sys::VmStat::from_str(&PROC_VMSTAT).unwrap();

        b.iter(move || {
            memory::sys::Swap::parse_str(black_box(PROC_MEMINFO), black_box(vm_stat.clone()))
        });
    });
}

criterion_group!(benches, bench_memory, bench_swap);
criterion_main!(benches);
