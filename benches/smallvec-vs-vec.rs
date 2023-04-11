//! Benchmark 4 word op on 64 bit arch.

use criterion::BatchSize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::ops::{Index, IndexMut};
// use smallvec::{SmallVec as LibSmallVec, smallvec};
use smallvec::{SmallVec as LibSmallVec, smallvec, smallvec_inline};
use paste::paste;
use std::ops::DerefMut;
use dhat;
use dhat::Profiler;
use tinyvec::{TinyVec as LibTinyVec, tiny_vec};

type SmallVec = LibSmallVec<[u8; 16]>;
type TinyVec = LibTinyVec<[u8; 16]>;

// ========== Build various datatypes =================

fn build_smallvec() -> SmallVec {
    // black_box(smallvec![0; 16])
    black_box(smallvec_inline![0; 16])
}

fn build_vec() -> Vec<u8> {
    black_box(vec![0; 16])
}

fn build_array() -> [u8; 16] {
    black_box([0; 16])
}

fn build_tinyvec() -> TinyVec {
    black_box(tiny_vec![0; 16])
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

/// Benchmark copy slice
macro_rules! bench_copy_slice_unknown_bounds {
    () => {
        |mut v| {
             for i in 0..10_000 {
                let start = black_box(0);
                let end = black_box(16);
                let value = black_box(i % 16);
                let slice = [value as u8; 16];
                v[start..end].copy_from_slice(&slice[..]);
            }
        }
    }
}
macro_rules! bench_copy_slice_known_bounds {
    () => {
      |mut v| {
           for i in 0..10_000 {
               let slice = [(i % 16) as u8; 16];
               v[0..16].copy_from_slice(&slice[..]);
           }
      }
    }
}

/// Benchmark copy slice
fn bench_copy_slice_known_bounds(mut v: impl DerefMut<Target=[u8]>) {
    for i in 0..10_000 {
        let slice = [(i % 16) as u8; 16];
        v[0..16].copy_from_slice(&slice[..]);
    }
}

/// Benchmark copy slice
fn bench_copy_slice_unknown_bounds(mut v: impl DerefMut<Target=[u8]>) {
    for i in 0..10_000 {
        let start = black_box(0);
        let end = black_box(16);
        let value = black_box(i % 16);
        let slice = [value as u8; 16];
        v[start..end].copy_from_slice(&slice[..]);
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
    bench_function!(c, array, bench_copy_slice_known_bounds!());
    bench_function!(c, array, bench_copy_slice_unknown_bounds!());
}

fn bench_smallvec(c: &mut Criterion) {
    bench_function!(c, smallvec, bench_inserts!());
    bench_function!(c, smallvec, bench_index);
    bench_function!(c, smallvec, bench_indexed_writes);
    bench_function!(c, smallvec, bench_copy_slice_known_bounds);
    bench_function!(c, smallvec, bench_copy_slice_unknown_bounds);
}

fn bench_vec(c: &mut Criterion) {
    bench_function!(c, vec, bench_inserts!());
    bench_function!(c, vec, bench_index);
    bench_function!(c, vec, bench_indexed_writes);
    bench_function!(c, vec, bench_copy_slice_known_bounds);
    bench_function!(c, vec, bench_copy_slice_unknown_bounds);
}


fn bench_tinyvec(c: &mut Criterion) {
    bench_function!(c, tinyvec, bench_inserts!());
    bench_function!(c, tinyvec, bench_index);
    bench_function!(c, tinyvec, bench_indexed_writes);
    bench_function!(c, tinyvec, bench_copy_slice_known_bounds);
    bench_function!(c, tinyvec, bench_copy_slice_unknown_bounds);
}

criterion_group!(benches, bench_tinyvec, bench_smallvec, bench_vec, bench_array);
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    benches();
    Criterion::default()
        .configure_from_args()
        .final_summary()
}
// criterion_main!(benches);
