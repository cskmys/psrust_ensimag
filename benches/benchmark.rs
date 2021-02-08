#[path = "../src/lib.rs"] mod lib;

use lib::{gen_2d_arr_rand, seq_iter_2d, seq_recur_2d, seq_recur_2d_opti};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn naive_benchmark(c: &mut Criterion) {
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("iterative", |b| b.iter(|| seq_iter_2d(&v)));
}

fn recursive_benchmark(c: &mut Criterion){
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("recursive", |b| b.iter(|| seq_recur_2d(&v)));
}

fn recursive_opti_benchmark(c: &mut Criterion){
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    c.bench_function("recursive_opti", |b| b.iter(|| seq_recur_2d_opti(&v)));
}

fn comp_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("comp");
    let v = gen_2d_arr_rand(4, 4, -100, 100);
    group.bench_with_input(BenchmarkId::new("iterative", 5), &v, |b, v| b.iter(|| seq_iter_2d(v)));
    group.bench_with_input(BenchmarkId::new("recursive", 5), &v, |b, v| b.iter(|| seq_recur_2d(v)));
    group.bench_with_input(BenchmarkId::new("recursive_opti", 5), &v, |b, v| b.iter(|| seq_recur_2d_opti(v)));
    group.finish();
}

// criterion_group!(benches, naive_benchmark, recursive_benchmark, recursive_opti_benchmark);
criterion_group!(benches, comp_benchmark);
criterion_main!(benches);