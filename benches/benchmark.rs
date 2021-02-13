#[path = "../src/lib.rs"] mod lib;

const L:i32 = 4;
const M:usize = 10000;
const N:usize = 10000;

const MIN:i32 = -10000;
const MAX:i32 = 10000;

use lib::{gen_2d_arr_rand, seq_iter_2d, seq_iter_2d_opti, seq_recur_2d, seq_recur_2d_opti, seq_recur_lev_2d, seq_recur_lev_2d_opti, par_iter_2d, par_iter_2d_opti, par_recur_2d, par_recur_2d_opti, par_recur_lev_2d, par_recur_lev_2d_opti, hyb_iter_2d, hyb_iter_2d_opti, hyb_recur_2d, hyb_recur_2d_opti, hyb_recur_lev_2d, hyb_recur_lev_2d_opti};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn comp_iter_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("01 iter");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("seq_iter", 5), &v, |b, v| b.iter(|| seq_iter_2d(v)));
    group.bench_with_input(BenchmarkId::new("par_iter", 5), &v, |b, v| b.iter(|| par_iter_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_iter", 5), &v, |b, v| b.iter(|| hyb_iter_2d(v)));
    group.finish();
}

fn comp_iter_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("02 iter_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("seq_iter_opti", 5), &v, |b, v| b.iter(|| seq_iter_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("par_iter_opti", 5), &v, |b, v| b.iter(|| par_iter_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("hyb_iter_opti", 5), &v, |b, v| b.iter(|| hyb_iter_2d_opti(v)));
    group.finish();
}

fn comp_iter_iter_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("03 iter_iter_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("hyb_iter", 5), &v, |b, v| b.iter(|| hyb_iter_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_iter_opti", 5), &v, |b, v| b.iter(|| hyb_iter_2d_opti(v)));
    group.finish();
}

fn comp_recur_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("04 recur");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("seq_recur", 5), &v, |b, v| b.iter(|| seq_recur_2d(v)));
    group.bench_with_input(BenchmarkId::new("par_recur", 5), &v, |b, v| b.iter(|| par_recur_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur", 5), &v, |b, v| b.iter(|| hyb_recur_2d(v)));
    group.finish();
}

fn comp_recur_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("05 recur_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("seq_recur_opti", 5), &v, |b, v| b.iter(|| seq_recur_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("par_recur_opti", 5), &v, |b, v| b.iter(|| par_recur_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_opti", 5), &v, |b, v| b.iter(|| hyb_recur_2d_opti(v)));
    group.finish();
}

fn comp_recur_recur_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("06 recur_recur_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    group.bench_with_input(BenchmarkId::new("hyb_recur", 5), &v, |b, v| b.iter(|| hyb_recur_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_opti", 5), &v, |b, v| b.iter(|| hyb_recur_2d_opti(v)));
    group.finish();
}

fn comp_recur_lev_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("07 recur_lev");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let lev = L;
    group.bench_with_input(BenchmarkId::new("seq_recur_lev", 5), &v, |b, v| b.iter(|| seq_recur_lev_2d(v, lev)));
    group.bench_with_input(BenchmarkId::new("par_recur_lev", 5), &v, |b, v| b.iter(|| par_recur_lev_2d(v, lev)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d(v, lev)));
    group.finish();
}

fn comp_recur_lev_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("08 recur_lev_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let lev = L;
    group.bench_with_input(BenchmarkId::new("seq_recur_lev_opti", 5), &v, |b, v| b.iter(|| seq_recur_lev_2d_opti(v, lev)));
    group.bench_with_input(BenchmarkId::new("par_recur_lev_opti", 5), &v, |b, v| b.iter(|| par_recur_lev_2d_opti(v, lev)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev_opti", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d_opti(v, lev)));
    group.finish();
}

fn comp_recur_lev_recur_lev_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("09 recur_lev_recur_lev_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let lev = L;
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d(v, lev)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev_opti", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d_opti(v, lev)));
    group.finish();
}

fn comp_iter_recur_recur_lev_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("10 iter_recur_recur_lev");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let lev = L;
    group.bench_with_input(BenchmarkId::new("hyb_iter", 5), &v, |b, v| b.iter(|| hyb_iter_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur", 5), &v, |b, v| b.iter(|| hyb_recur_2d(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d(v, lev)));
    group.finish();
}

fn comp_iter_recur_recur_lev_opti_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("11 iter_recur_recur_lev_opti");
    let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let lev = L;
    group.bench_with_input(BenchmarkId::new("hyb_iter_opti", 5), &v, |b, v| b.iter(|| hyb_iter_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_opti", 5), &v, |b, v| b.iter(|| hyb_recur_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("hyb_recur_lev_opti", 5), &v, |b, v| b.iter(|| hyb_recur_lev_2d_opti(v, lev)));
    group.finish();
}

criterion_group!(benches, comp_iter_benchmark, comp_iter_opti_benchmark, comp_iter_iter_opti_benchmark, comp_recur_benchmark, comp_recur_opti_benchmark, comp_recur_recur_opti_benchmark, comp_recur_lev_benchmark, comp_recur_lev_opti_benchmark, comp_recur_lev_recur_lev_opti_benchmark, comp_iter_recur_recur_lev_benchmark, comp_iter_recur_recur_lev_opti_benchmark);
criterion_main!(benches);