//! Benchmark 4 word op on 64 bit arch.

#![feature(portable_simd)]

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
use std::simd::*;

type Item = u8;
pub fn compare(v1: Vec<Item>, v2: Vec<Item>) -> bool {
    v1 == v2
}

pub fn compare2(v1: Vec<Item>, v2: Vec<Item>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    return true;
}

pub fn compare3(v1: Vec<Simd<Item, 16>>, v2: Vec<Simd<Item, 16>>) -> bool {
    v1 == v2

}

pub fn compare4(v1: Vec<u64>, v2: Vec<u64>) -> bool {
    v1 == v2

}

pub fn bench_compare3(c: &mut Criterion) {
    let setup_3_diff = || {
        let v1 = black_box(vec![black_box(0); black_box(1000)]);
        let v2 = black_box(vec![black_box(1); black_box(1000)]);
        let mut s1: Vec<Simd<Item, 16>> = vec![Simd::from_array([0; 16]); v1.len()/16];
        let mut s2: Vec<Simd<Item, 16>> = vec![Simd::from_array([1; 16]); v1.len()/16];
        for i in 0..v1.len() / 16 {
            s1[i] = Simd::from_slice(&v1[i..i+16]);
            s2[i] = Simd::from_slice(&v2[i..i+16]);
        }
        (s1, s2)
    };
    let setup_3_same = || {
        let v1 = black_box(vec![black_box(0); black_box(1000)]);
        let v2 = black_box(vec![black_box(0); black_box(1000)]);
        let mut s1: Vec<Simd<Item, 16>> = vec![Simd::from_array([0; 16]); v1.len()/16];
        let mut s2: Vec<Simd<Item, 16>> = vec![Simd::from_array([0; 16]); v1.len()/16];
        for i in 0..v1.len() / 16 {
            s1[i] = Simd::from_slice(&v1[i..i+16]);
            s2[i] = Simd::from_slice(&v2[i..i+16]);
        }
        (s1, s2)
    };
    c.bench_function("compare3 diff", |b| {
        b.iter_batched(
            setup_3_diff,
            | (v1, v2) | compare3(v1, v2),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("compare3 same", |b| {
        b.iter_batched(
            setup_3_same,
            | (v1, v2) | compare3(v1, v2),
            BatchSize::SmallInput,
        )
    });
}

pub fn bench_compare4(c: &mut Criterion) {
    let setup_3_diff = || {
        let v1 = black_box(vec![black_box(0); black_box(1000/8)]);
        let v2 = black_box(vec![black_box(1); black_box(1000/8)]);
        (v1, v2)
    };
    let setup_3_same = || {
        let v1 = black_box(vec![black_box(0); black_box(1000/8)]);
        let v2 = black_box(vec![black_box(0); black_box(1000/8)]);
        (v1, v2)
    };
    c.bench_function("compare4 diff", |b| {
        b.iter_batched(
            setup_3_diff,
            | (v1, v2) | compare4(v1, v2),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("compare4 same", |b| {
        b.iter_batched(
            setup_3_same,
            | (v1, v2) | compare4(v1, v2),
            BatchSize::SmallInput,
        )
    });
}

macro_rules! make_bench {
    ($name:ident) => {
        paste!{
             pub fn [<bench_ $name>](c: &mut Criterion) {
                c.bench_function(stringify!($name different), |b| {
                    b.iter_batched(
                        || (black_box(vec![black_box(0); black_box(1000)]),
                            black_box(vec![black_box(1); black_box(1000)])),
                        | (v1, v2) | $name(v1, v2),
                        BatchSize::SmallInput,
                    )
                });
                c.bench_function(stringify!($name same), |b| {
                    b.iter_batched(
                        || (black_box(vec![black_box(1); black_box(1000)]),
                            black_box(vec![black_box(1); black_box(1000)])),
                        | (v1, v2) | $name(v1, v2),
                        BatchSize::SmallInput,
                    )
                });
            }
        }
    }
}

make_bench!(compare);
make_bench!(compare2);

criterion_group!(benches, bench_compare, bench_compare2, bench_compare3, bench_compare4);
fn main() {
    benches();
    Criterion::default()
        .configure_from_args()
        .final_summary()
}
// criterion_main!(benches);
