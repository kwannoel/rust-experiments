use criterion::black_box;
use criterion::BatchSize;
use criterion::{criterion_group, criterion_main, Criterion};
use std::mem::ManuallyDrop;

#[derive(Copy, Clone, Debug)]
pub struct A {
    inner: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct B {
    inner: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct C {
    inner: u32,
}

#[derive(Clone, Debug)]
pub struct As {
    inner: Vec<A>,
}

impl As {
    fn append(&mut self, x: A) {
        self.inner.push(x)
    }

    fn append_u(&mut self, x: DynU) {
        unsafe { self.inner.push(x.a) }
    }

    fn append_d(&mut self, x: Dyn) {
        unsafe { self.inner.push(x.a()) }
    }
}

#[derive(Clone, Debug)]
pub struct Bs {
    inner: Vec<B>,
}

impl Bs {
    fn append(&mut self, x: B) {
        self.inner.push(x)
    }
    fn append_u(&mut self, x: DynU) {
        unsafe { self.inner.push(x.b) }
    }

    fn append_d(&mut self, x: Dyn) {
        unsafe { self.inner.push(x.b()) }
    }
}

#[derive(Clone, Debug)]
pub struct Cs {
    inner: Vec<C>,
}

impl Cs {
    fn append(&mut self, x: C) {
        self.inner.push(x)
    }
    fn append_u(&mut self, x: DynU) {
        unsafe { self.inner.push(x.c) }
    }

    fn append_d(&mut self, x: Dyn) {
        unsafe { self.inner.push(x.c()) }
    }
}

#[derive(Clone, Debug)]
enum Dyn {
    A(A),
    B(B),
    C(C),
}

impl Dyn {
    fn a(self) -> A {
        if let Self::A(a) = self {
            a
        } else {
            unreachable!()
        }
    }
    fn b(self) -> B {
        if let Self::B(b) = self {
            b
        } else {
            unreachable!()
        }
    }
    fn c(self) -> C {
        if let Self::C(c) = self {
            c
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
enum DynE {
    A,
    B,
    C,
}

union DynU {
    a: A,
    b: B,
    c: C,
}

#[derive(Clone, Debug)]
enum Dyns {
    A(As),
    B(Bs),
    C(Cs),
}

impl Dyns {
    fn make_a() -> Self {
        black_box(Dyns::A(As {
            inner: vec![A { inner: 123 }; 10],
        }))
    }
    fn make_b() -> Self {
        black_box(Dyns::B(Bs {
            inner: vec![B { inner: 123 }; 10],
        }))
    }
    fn make_c() -> Self {
        black_box(Dyns::C(Cs {
            inner: vec![C { inner: 123 }; 10],
        }))
    }
    fn append_no_info(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => xs.append(x),
            (Dyns::B(xs), Dyn::B(x)) => xs.append(x),
            (Dyns::C(xs), Dyn::C(x)) => xs.append(x),
            (_, _) => {
                panic!("failed...")
            }
        }
    }

    fn append_with_info(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => xs.append(x),
            (Dyns::B(xs), Dyn::B(x)) => xs.append(x),
            (Dyns::C(xs), Dyn::C(x)) => xs.append(x),
            (a, b) => {
                panic!("failed...{:?} {:?}", a, b)
            }
        }
    }

    fn append_partial_match_arm(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => xs.append(x),
            (Dyns::B(xs), Dyn::B(x)) => xs.append(x),
            (Dyns::C(xs), Dyn::C(x)) => xs.append(x),
            (a, b) => unreachable!(),
        }
    }

    fn append_unsafe(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => xs.append(x),
            (_, _) => {
                panic!("failed...")
            }
        }
    }

    fn append_u(&mut self, x: DynU) {
        match self {
            Dyns::A(xs) => xs.append_u(x),
            Dyns::B(xs) => xs.append_u(x),
            Dyns::C(xs) => xs.append_u(x),
            _ => unreachable!(),
        }
    }

    fn append_d(&mut self, x: Dyn) {
        match self {
            Dyns::A(xs) => xs.append_d(x),
            Dyns::B(xs) => xs.append_d(x),
            Dyns::C(xs) => xs.append_d(x),
            _ => unreachable!(),
        }
    }
}

union DynsU {
    a: ManuallyDrop<As>,
    b: ManuallyDrop<Bs>,
    c: ManuallyDrop<Cs>,
}

impl DynsU {
    fn make_a() -> Self {
        black_box(DynsU {
            a: ManuallyDrop::new(As {
                inner: vec![A { inner: 123 }; 10],
            }),
        })
    }
    fn make_b() -> Self {
        black_box(DynsU {
            b: ManuallyDrop::new(Bs {
                inner: vec![B { inner: 123 }; 10],
            }),
        })
    }
    fn make_c() -> Self {
        black_box(DynsU {
            c: ManuallyDrop::new(Cs {
                inner: vec![C { inner: 123 }; 10],
            }),
        })
    }

    fn append_u(&mut self, d: DynE, x: DynU) {
        match d {
            DynE::A => unsafe { self.a.append_u(x) },
            DynE::B => unsafe { self.b.append_u(x) },
            DynE::C => unsafe { self.c.append_u(x) },
        }
    }
}

pub fn bench(c: &mut Criterion) {
    c.bench_function("dyn_no_info", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_no_info(Dyn::A(A { inner: 123 })));
                    black_box(ys.append_no_info(Dyn::B(B { inner: 333 })));
                    black_box(zs.append_no_info(Dyn::C(C { inner: 444 })));
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("dyn_partial", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_partial_match_arm(Dyn::A(A { inner: 123 })));
                    black_box(ys.append_partial_match_arm(Dyn::B(B { inner: 333 })));
                    black_box(zs.append_partial_match_arm(Dyn::C(C { inner: 444 })));
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("unsafe", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_a(), Dyns::make_a()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_partial_match_arm(Dyn::A(A { inner: 123 })));
                    black_box(ys.append_partial_match_arm(Dyn::A(A { inner: 333 })));
                    black_box(zs.append_partial_match_arm(Dyn::A(A { inner: 444 })));
                }
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("union dyn", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_u(DynU {
                        a: A { inner: 123 },
                    }));
                    black_box(ys.append_u(DynU {
                        b: B { inner: 333 },
                    }));
                    black_box(zs.append_u(DynU {
                        c: C { inner: 444 },
                    }));
                }
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("downcast", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_d(Dyn::A(A { inner: 123 })));
                    black_box(ys.append_d(Dyn::B(B { inner: 333 })));
                    black_box(zs.append_d(Dyn::C(C { inner: 444 })));
                }
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("union dyns and dyn outer match", |b| {
        b.iter_batched(
            || {
                (
                    (DynE::A, DynsU::make_a()),
                    (DynE::B, DynsU::make_b()),
                    (DynE::C, DynsU::make_c()),
                )
            },
            |((da, mut xs), (db, mut ys), (dc, mut zs))| {
                for i in 0..black_box(10) {
                    black_box(xs.append_u(
                        da,
                        DynU {
                            a: A { inner: 123 },
                        },
                    ));
                    black_box(ys.append_u(
                        db,
                        DynU {
                            b: B { inner: 333 },
                        },
                    ));
                    black_box(zs.append_u(
                        dc,
                        DynU {
                            c: C { inner: 444 },
                        },
                    ));
                }
                black_box(drop(xs));
                black_box(drop(ys));
                black_box(drop(zs));
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("union dyns and dyn", |b| {
        b.iter_batched(
            || (DynsU::make_a(), DynsU::make_b(), DynsU::make_c()),
            |(mut xs, mut ys, mut zs)| {
                for i in 0..black_box(10) {
                    black_box(xs.append_u(
                        DynE::A,
                        DynU {
                            a: A { inner: 123 },
                        },
                    ));
                    black_box(ys.append_u(
                        DynE::B,
                        DynU {
                            b: B { inner: 333 },
                        },
                    ));
                    black_box(zs.append_u(
                        DynE::C,
                        DynU {
                            c: C { inner: 444 },
                        },
                    ));
                }
                black_box(drop(xs));
                black_box(drop(ys));
                black_box(drop(zs));
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("union dyns and dyn no disc", |b| {
        b.iter_batched(
            || (DynsU::make_a(), DynsU::make_b(), DynsU::make_c()),
            |(mut xs, mut ys, mut zs)| {
                unsafe {
                    for i in 0..black_box(10) {
                        black_box((*xs.a).append_u(DynU {
                            a: A { inner: 123 },
                        }));
                        black_box((*ys.b).append_u(DynU {
                            b: B { inner: 333 },
                        }));
                        black_box((*zs.c).append_u(DynU {
                            c: C { inner: 444 },
                        }));
                    }
                }
                black_box(drop(xs));
                black_box(drop(ys));
                black_box(drop(zs));
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
