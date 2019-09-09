#[macro_use]
extern crate criterion;

use criterion::Criterion;

//use heim_common::prelude::*;
use heim_cpu as cpu;

fn bench_cpu(c: &mut Criterion) {
    let mut executor = futures_executor::ThreadPool::new().unwrap();

    c.bench_function("logical count", |b| {
        b.iter(|| executor.run(cpu::logical_count()));
    });

    c.bench_function("physical count", |b| {
        b.iter(|| executor.run(cpu::physical_count()));
    });

    //    c.bench_function("platform", |b| {
    //        b.iter(|| executor.run(host::platform()));
    //    });
    //
    //    c.bench_function("users", |b| {
    //        b.iter(|| {
    //            let stream = host::users().for_each(|_| future::ready(()));
    //            executor.run(stream)
    //        });
    //    });
}

criterion_group!(benches, bench_cpu);
criterion_main!(benches);

//#[heim_derive::bench]
//async fn bench_frequency() {
//    cpu::frequency().await
//}
//
//#[heim_derive::bench]
//async fn bench_logical_count() {
//    cpu::logical_count().await
//}
//
//#[heim_derive::bench]
//async fn bench_physical_count() {
//    cpu::physical_count().await
//}
//
//#[heim_derive::bench]
//async fn bench_stats() {
//    cpu::stats().await
//}
//
//#[heim_derive::bench]
//async fn bench_time() {
//    cpu::time().await
//}
//
//#[heim_derive::bench]
//async fn bench_times() {
//    let stream = cpu::times().for_each(|_| future::ready(()));
//
//    stream.await
//}
