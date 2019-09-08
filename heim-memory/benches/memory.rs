#[macro_use]
extern crate criterion;

use std::str::FromStr;

use criterion::black_box;
use criterion::Criterion;

use heim_memory as memory;

fn bench_memory(c: &mut Criterion) {
    let mut executor = futures_executor::ThreadPool::new().unwrap();

    c.bench_function("memory", |b| {
        b.iter(|| executor.run(memory::memory()));
    });

    c.bench_function("swap", |b| {
        b.iter(|| executor.run(memory::swap()));
    });
}

criterion_group!(benches, bench_memory);
criterion_main!(benches);
