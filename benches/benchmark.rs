#[path = "../src/lib.rs"] mod lib;

const L:i32 = 8;
const M:usize = 10000;
const N:usize = 10000;

const MIN:i32 = -10000;
const MAX:i32 = 10000;

use lib::{gen_2d_arr_rand, gen_2d_arr_uni, iter_seq_2d, iter_seq_2d_opti, recur_seq_2d, recur_seq_2d_opti, iter_par_2d, iter_par_2d_opti, recur_par_2d, recur_par_2d_opti, recur_cwd_2d, recur_cwd_2d_opti, iter_hyb_2d, iter_hyb_2d_opti, recur_hyb_2d, recur_hyb_2d_opti};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn comp_iter_benchmark(c: &mut Criterion, v: &[Vec<i32>]){
    let mut group = c.benchmark_group("01 iter");
    group.bench_with_input(BenchmarkId::new("iter_seq", 5), v, |b, v| b.iter(|| iter_seq_2d(v)));
    group.bench_with_input(BenchmarkId::new("iter_par", 5), v, |b, v| b.iter(|| iter_par_2d(v)));
    group.bench_with_input(BenchmarkId::new("iter_hyb", 5), v, |b, v| b.iter(|| iter_hyb_2d(v)));
    group.finish();
}

fn comp_iter_opti_benchmark(c: &mut Criterion, v: &[Vec<i32>]){
    let mut group = c.benchmark_group("02 iter_opti");
    group.bench_with_input(BenchmarkId::new("iter_seq_opti", 5), v, |b, v| b.iter(|| iter_seq_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("iter_par_opti", 5), v, |b, v| b.iter(|| iter_par_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("iter_hyb_opti", 5), v, |b, v| b.iter(|| iter_hyb_2d_opti(v)));
    group.finish();
}

fn comp_iter_iter_opti_benchmark(c: &mut Criterion, v: &[Vec<i32>]){
    let mut group = c.benchmark_group("03 iter_iter_opti");
    group.bench_with_input(BenchmarkId::new("iter_hyb", 5), v, |b, v| b.iter(|| iter_hyb_2d(v)));
    group.bench_with_input(BenchmarkId::new("iter_hyb_opti", 5), v, |b, v| b.iter(|| iter_hyb_2d_opti(v)));
    group.finish();
}

fn comp_recur_benchmark(c: &mut Criterion, v: &[Vec<i32>], lev:i32){
    let mut group = c.benchmark_group("04 recur");
    group.bench_with_input(BenchmarkId::new("recur_seq", 5), v, |b, v| b.iter(|| recur_seq_2d(v)));
    group.bench_with_input(BenchmarkId::new("recur_par", 5), v, |b, v| b.iter(|| recur_par_2d(v)));
    group.bench_with_input(BenchmarkId::new("recur_hyb", 5), v, |b, v| b.iter(|| recur_hyb_2d(v)));
    group.bench_with_input(BenchmarkId::new("recur_cwd", 5), v, |b, v| b.iter(|| recur_cwd_2d(v, lev)));
    group.finish();
}

fn comp_recur_opti_benchmark(c: &mut Criterion, v: &[Vec<i32>], lev:i32){
    let mut group = c.benchmark_group("05 recur_opti");
    group.bench_with_input(BenchmarkId::new("recur_seq_opti", 5), v, |b, v| b.iter(|| recur_seq_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("recur_par_opti", 5), v, |b, v| b.iter(|| recur_par_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("recur_hyb_opti", 5), v, |b, v| b.iter(|| recur_hyb_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("recur_cwd_opti", 5), v, |b, v| b.iter(|| recur_cwd_2d_opti(v, lev)));
    group.finish();
}

fn comp_recur_recur_opti_benchmark(c: &mut Criterion, v: &[Vec<i32>], lev:i32){
    let mut group = c.benchmark_group("06 recur_recur_opti");
    group.bench_with_input(BenchmarkId::new("recur_cwd", 5), v, |b, v| b.iter(|| recur_cwd_2d(v, lev)));
    group.bench_with_input(BenchmarkId::new("recur_cwd_opti", 5), v, |b, v| b.iter(|| recur_cwd_2d_opti(v, lev)));
    group.finish();
}

fn comp_iter_recur_benchmark(c: &mut Criterion, v: &[Vec<i32>], lev:i32){
    let mut group = c.benchmark_group("07 iter_recur");
    group.bench_with_input(BenchmarkId::new("iter_hyb", 5), v, |b, v| b.iter(|| iter_hyb_2d(v)));
    group.bench_with_input(BenchmarkId::new("recur_cwd", 5), v, |b, v| b.iter(|| recur_cwd_2d(v, lev)));
    group.finish();
}

fn comp_iter_recur_opti_benchmark(c: &mut Criterion, v: &[Vec<i32>], lev:i32){
    let mut group = c.benchmark_group("08 iter_recur_opti");
    group.bench_with_input(BenchmarkId::new("iter_hyb_opti", 5), v, |b, v| b.iter(|| iter_hyb_2d_opti(v)));
    group.bench_with_input(BenchmarkId::new("recur_cwd_opti", 5), v, |b, v| b.iter(|| recur_cwd_2d_opti(v, lev)));
    group.finish();
}

fn bench(c: &mut Criterion){
    // let v = gen_2d_arr_rand(M, N, MIN, MAX);
    let v = gen_2d_arr_uni(M, N);
    let lev = L;
    comp_iter_benchmark(c, &v);
    comp_iter_opti_benchmark(c, &v);
    comp_iter_iter_opti_benchmark(c, &v);
    comp_recur_benchmark(c, &v, lev);
    comp_recur_opti_benchmark(c, &v, lev);
    comp_recur_recur_opti_benchmark(c, &v, lev);
    comp_iter_recur_benchmark(c, &v, lev);
    comp_iter_recur_opti_benchmark(c, &v, lev);
}

criterion_group!(benches, bench);
criterion_main!(benches);