//! Benchmark 4 word op on 64 bit arch.

use criterion::BatchSize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ops::{Index, IndexMut};
use smallvec::{SmallVec as LibSmallVec, smallvec};
use paste::paste;

type SmallVec = LibSmallVec<[u8; 16]>;

// ========== Build various datatypes =================

fn build_smallvec() -> SmallVec {
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
macro_rules! bench_inserts {
    () => {
        |mut v| {
            for i in 0..16 {
                v.push(black_box(i as u8));
            }
        }
    }
}

/// Handle benchmark boilerplate.
macro_rules! bench_function {
    ($criterion:ident, $name:ident, $method:expr) => {
        $criterion.bench_function(stringify!($name $method), paste!{
            bench_method!([<build_ $name>], $method)
        })
    }
}

/// Handle benchmark boilerplate.
macro_rules! bench_method {
    ($builder:ident, $method:expr) => {
      |b| {
            b.iter_batched(
                $builder,
                $method,
                BatchSize::SmallInput,
            )
        }
    }
}

// ========== Benchmark various datatypes =================

fn bench_array(c: &mut Criterion) {
    bench_function!(c, array, bench_indexed_writes);
}

fn bench_smallvec(c: &mut Criterion) {
    bench_function!(c, smallvec, bench_inserts!());
    bench_function!(c, smallvec, bench_index);
    bench_function!(c, smallvec, bench_indexed_writes);
    // bench_function!(c, "smallvec", build_smallvec, bench_copy_slice);
}

fn bench_vec(c: &mut Criterion) {
    bench_function!(c, vec, bench_inserts!());
    bench_function!(c, vec, bench_index);
    bench_function!(c, vec, bench_indexed_writes);
}

criterion_group!(benches, bench_smallvec, bench_vec, bench_array);
criterion_main!(benches);
