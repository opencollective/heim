#[macro_use]
extern crate criterion;

use criterion::Criterion;

use heim_common::prelude::*;
use heim_host as host;

fn bench_host(c: &mut Criterion) {
    let mut executor = futures_executor::ThreadPool::new().unwrap();

    c.bench_function("boot_time", |b| {
        b.iter(|| executor.run(host::boot_time()));
    });

    c.bench_function("uptime", |b| {
        b.iter(|| executor.run(host::uptime()));
    });

    c.bench_function("platform", |b| {
        b.iter(|| executor.run(host::platform()));
    });

    c.bench_function("users", |b| {
        b.iter(|| {
            let stream = host::users().for_each(|_| future::ready(()));
            executor.run(stream)
        });
    });
}

criterion_group!(benches, bench_host);
criterion_main!(benches);
