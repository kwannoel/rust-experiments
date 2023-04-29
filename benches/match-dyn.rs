use criterion::black_box;
use criterion::BatchSize;
use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Clone, Debug)]
pub struct A {
    inner: usize,
}

impl A {
    fn from_dyn(Dyn::A(a): Dyn) -> Self {
        a
    }
}

#[derive(Clone, Debug)]
pub struct B {
    inner: u64,
}

#[derive(Clone, Debug)]
pub struct C {
    inner: u32,
}

#[derive(Clone, Debug)]
pub struct As {
    inner: Vec<A>
}

impl As {
    fn append(&mut self, x: A) {
        self.inner.push(x)
    }

    fn from_dyns(Dyns::A(a): Dyns) -> Self {
        a
    }
}

#[derive(Clone, Debug)]
pub struct Bs {
    inner: Vec<B>
}

impl Bs {
    fn append(&mut self, x: B) {
        self.inner.push(x)
    }
}

#[derive(Clone, Debug)]
pub struct Cs {
    inner: Vec<C>
}

impl Cs {
    fn append(&mut self, x: C) {
        self.inner.push(x)
    }
}

#[derive(Clone, Debug)]
enum Dyn {
    A(A),
    B(B),
    C(C),
}

#[derive(Clone, Debug)]
enum Dyns {
    A(As),
    B(Bs),
    C(Cs),
}

impl Dyns {
    fn make_a() -> Self {
        black_box(Dyns::A(As { inner: vec![A { inner: 123}; 10]}))
    }
    fn make_b() -> Self {
        black_box(Dyns::B(Bs { inner: vec![B { inner: 123 }; 10]}))
    }
    fn make_c() -> Self {
        black_box(Dyns::C(Cs { inner: vec![C { inner: 123 }; 10]}))
    }
    fn append_no_info(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => { xs.append(x) },
            (Dyns::B(xs), Dyn::B(x)) => { xs.append(x) },
            (Dyns::C(xs), Dyn::C(x)) => { xs.append(x) },
            (_, _) => { panic!("failed...") }
        }
    }

    fn append_with_info(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => { xs.append(x) },
            (Dyns::B(xs), Dyn::B(x)) => { xs.append(x) },
            (Dyns::C(xs), Dyn::C(x)) => { xs.append(x) },
            (a, b) => { panic!("failed...{:?} {:?}", a, b) }
        }
    }

    fn append_partial_match_arm(&mut self, x: Dyn) {
        match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => { xs.append(x) },
            (Dyns::B(xs), Dyn::B(x)) => { xs.append(x) },
            (Dyns::C(xs), Dyn::C(x)) => { xs.append(x) },
            (a, b) => unreachable!()
        }
    }

    fn append_unsafe(&mut self, x: Dyn) {
         match (self, x) {
            (Dyns::A(xs), Dyn::A(x)) => { xs.append(x) },
            (_, _) => { panic!("failed...") }
        }
    }
}

pub fn bench(c: &mut Criterion) {
    c.bench_function("dyn_no_info", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| for i in 0..black_box(10) {
                black_box(xs.append_no_info(Dyn::A(A { inner: 123 })));
                black_box(ys.append_no_info(Dyn::B(B { inner: 333 })));
                black_box(zs.append_no_info(Dyn::C(C { inner: 444 })));
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("dyn_partial", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_b(), Dyns::make_c()),
            |(mut xs, mut ys, mut zs)| for i in 0..black_box(10) {
                black_box(xs.append_partial_match_arm(Dyn::A(A { inner: 123 })));
                black_box(ys.append_partial_match_arm(Dyn::B(B { inner: 333 })));
                black_box(zs.append_partial_match_arm(Dyn::C(C { inner: 444 })));
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("unsafe", |b| {
        b.iter_batched(
            || (Dyns::make_a(), Dyns::make_a(), Dyns::make_a()),
            |(mut xs, mut ys, mut zs)| for i in 0..black_box(10) {
                black_box(xs.append_partial_match_arm(Dyn::A(A { inner: 123 })));
                black_box(ys.append_partial_match_arm(Dyn::A(A { inner: 123 })));
                black_box(zs.append_partial_match_arm(Dyn::A(A { inner: 123 })));
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
