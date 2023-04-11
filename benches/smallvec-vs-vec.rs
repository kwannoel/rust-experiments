//! Benchmark 4 word op on 64 bit arch.

use criterion::BatchSize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ops::{Index, IndexMut};
use smallvec::{SmallVec as LibSmallVec, smallvec};

type SmallVec = LibSmallVec<[u8; 16]>;

// ========== Build various datatypes =================

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

// ========== Benchmark methods =================

/// Benchmark index op.
fn bench_index(v: impl Index<usize, Output=u8>) {
    for i in 0..10_000 {
        black_box(v[black_box(i%16)]);
    }
}

/// Benchmark assign at index.
fn bench_indexed_writes(mut v: impl AsMut<[u8]> + IndexMut<usize, Output=u8>) {
    for i in 0..10_000 {
        v[black_box(i%16)] = black_box(i as u8);
    }
}

/// For struct with `push`, creates a closure for it, there is no trait for it.
macro_rules! bench_insert {
    () => {
        |mut v| {
            for i in 0..16 {
                v.push(black_box(i as u8));
            }
        }
    }
}

// ========== Benchmark various datatypes =================

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
            build_small_vec,
            bench_insert!(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("smallvec indexing", |b| {
        b.iter_batched(
            build_small_vec,
            bench_index,
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
            build_vec,
            bench_insert!(),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("vec indexing", |b| {
        b.iter_batched(
            build_vec,
            bench_index,
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
