use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_gen_const_vs_lazy_static::ls;
use rust_gen_const_vs_lazy_static::hc;
use rust_gen_const_vs_lazy_static::mc;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trib 5.1 ls", |b| b.iter(|| ls::tribonacci(black_box(5.1))));
    c.bench_function("trib 5.1 hc", |b| b.iter(|| hc::tribonacci(black_box(5.1))));
    c.bench_function("trib 5.1 mc", |b| b.iter(|| mc::tribonacci(black_box(5.1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);