#[path = "../src/lib.rs"] mod lib;

use criterion::{criterion_group, criterion_main, Criterion};

fn naive_benchmark(c: &mut Criterion) {
    let v = lib::gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("naive", |b| b.iter(|| lib::naive_version_2d(&v)));
}

criterion_group!(benches, naive_benchmark);

criterion_main!(benches);