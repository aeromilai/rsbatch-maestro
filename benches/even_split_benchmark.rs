// benches/even_split_benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use batch_maestro::even_split;

fn benchmark_even_split(c: &mut Criterion) {
    c.bench_function("even_split 1000000 items", |b| {
        b.iter(|| even_split(black_box(1_000_000), black_box(1000)))
    });

    c.bench_function("even_split prime number", |b| {
        b.iter(|| even_split(black_box(9973), black_box(100)))
    });

    c.bench_function("even_split small number", |b| {
        b.iter(|| even_split(black_box(50), black_box(8)))
    });
}

criterion_group!(benches, benchmark_even_split);
criterion_main!(benches);
