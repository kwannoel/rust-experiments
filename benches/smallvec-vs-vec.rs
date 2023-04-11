//! Benchmark 4 word op on 64 bit arch.

use criterion::BatchSize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ops::IndexMut;
use smallvec::{SmallVec as LibSmallVec, smallvec};

type SmallVec = LibSmallVec<[u8; 16]>;

fn build_small_vec() -> SmallVec {
    let sz = black_box(16);
    black_box(smallvec![0; sz])
}

fn build_vec() -> Vec<u8> {
    let sz = black_box(16);
    black_box(vec![0; sz])
}

fn build_array() -> [u8; 16] {
    black_box([0; 16])
}

fn bench_indexed_writes(mut v: impl AsMut<[u8]> + IndexMut<usize, Output=u8>) {
    for i in 0..16 {
        v[i] = black_box(16-(i as u8));
    }
}

fn bench_array(c: &mut Criterion) {
    c.bench_function("array indexed writes", |b| {
        b.iter_batched(
            build_array,
            bench_indexed_writes,
            BatchSize::SmallInput,
        )
    });
}

fn bench_smallvec(c: &mut Criterion) {
    c.bench_function("smallvec inserts", |b| {
        b.iter_batched(
            || {
                let v: SmallVec = black_box(SmallVec::with_capacity(16));
                v
            },
            |mut v| {
                for i in 0..16 {
                    v.push(black_box(i as u8));
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("smallvec indexing", |b| {
        b.iter_batched(
            || {
                let v: SmallVec = black_box(smallvec![123; 16]);
                v
            },
            |v| {
                for i in 0..16 {
                    black_box(v[i]);
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("smallvec indexed writes", |b| {
        b.iter_batched(
            build_small_vec,
            bench_indexed_writes,
            BatchSize::SmallInput,
        )
    });
}

fn bench_vec(c: &mut Criterion) {
    c.bench_function("vec inserts", |b| {
        b.iter_batched(
            || {
                let v: Vec<u8> = black_box(Vec::with_capacity(16));
                v
            },
            |mut v| {
                for i in 0..16 {
                    v.push(black_box(i as u8));
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("vec indexing", |b| {
        b.iter_batched(
            || {
                let v: Vec<u8> = black_box(vec![123; 16]);
                v
            },
            |v| {
                for i in 0..16 {
                    black_box(v[i]);
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("vec indexed writes", |b| {
        b.iter_batched(
            build_vec,
            bench_indexed_writes,
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_smallvec, bench_vec, bench_array);
criterion_main!(benches);
