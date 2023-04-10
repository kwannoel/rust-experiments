//! Benchmark 4 word op on 64 bit arch.

use criterion::BatchSize;
use criterion::{Criterion, criterion_group, criterion_main};
use criterion::black_box;
use smallvec::SmallVec;

fn bench_array(c: &mut Criterion) {
     c.bench_function("array inserts", |b| b.iter_batched(|| {
        let v: [u8; 32] = black_box([0; 32]);
        v
    }, |mut v| { for i in 0..32 {
         v[i] = black_box(i as u8);
    }
    }, BatchSize::SmallInput));
}

fn bench_smallvec(c: &mut Criterion) {
    c.bench_function("smallvec inserts", |b| b.iter_batched(|| {
        let v: SmallVec<[u8; 32]> = black_box(SmallVec::with_capacity(64 * 4));
        v
    }, |mut v| { for i in 0..32 {
        v.push(black_box(i as u8));
    }
    }, BatchSize::SmallInput));
        c.bench_function("smallvec indexing", |b| b.iter_batched(|| {
        let mut v: SmallVec<[u8; 32]> = black_box(SmallVec::with_capacity(64 * 4));
        for i in 0..32 {
            v.push(black_box(i as u8));
        }
        v
    }, |mut v| { for i in 0..32 {
        black_box(v[i]);
    }
    }, BatchSize::SmallInput));
}

fn bench_vec(c: &mut Criterion) {
    c.bench_function("vec inserts", |b| b.iter_batched(|| {
        let v: Vec<u8> = black_box(Vec::with_capacity(64 * 4));
        v
    }, |mut v| { for i in 0..32 {
        v.push(black_box(i as u8));
    }
    }, BatchSize::SmallInput));
    c.bench_function("vec indexing", |b| b.iter_batched(|| {
        let mut v: Vec<u8> = black_box(vec![123; 32]);
        for i in 0..32 {
            v.push(black_box(i as u8));
        }
        v
    }, |v| { for i in 0..32 {
        black_box(v[i]);
    }}, BatchSize::SmallInput));
}

criterion_group!(benches, bench_smallvec, bench_vec, bench_array);
criterion_main!(benches);
