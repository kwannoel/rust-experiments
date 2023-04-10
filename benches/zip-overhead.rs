use criterion::BatchSize;
use criterion::{Criterion, criterion_group, criterion_main};
use criterion::black_box;

fn bench_method1(c: &mut Criterion) {
    c.bench_function("with_zip", |b| b.iter_batched(|| {
        black_box((vec![10; 1_000_000], vec![20; 1_000_000]))
    }, |(a, b)| {
        for (x, y) in a.iter().zip(b.iter()) {
            black_box(x + y);
        }
    }, BatchSize::SmallInput));
}

fn bench_method2(c: &mut Criterion) {
    c.bench_function("without_zip", |b| b.iter_batched(|| {
        black_box((vec![10; 1_000_000], vec![20; 1_000_000]))
    }, |(a, b)| {
        for i in 0..a.len() {
            black_box(a[i] + b[i]);
        }
    }, BatchSize::SmallInput));
}

criterion_group!(benches, bench_method1, bench_method2);
criterion_main!(benches);
