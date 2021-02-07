#[path = "../src/lib.rs"] mod lib;

use lib::{gen_2d_arr_rand, naive_version_2d, recursive_version_2d, recursive_version_2d_opti};

use criterion::{criterion_group, criterion_main, Criterion};

fn naive_benchmark(c: &mut Criterion) {
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("naive", |b| b.iter(|| naive_version_2d(&v)));
}

fn recursive_benchmark(c: &mut Criterion){
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("recursive", |b| b.iter(|| recursive_version_2d(&v)));
}

criterion_group!(benches, naive_benchmark, recursive_benchmark);
criterion_main!(benches);