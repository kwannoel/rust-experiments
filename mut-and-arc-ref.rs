use std::sync::Arc;

struct A {
    immut: Arc<usize>,
    mutt: usize,
}

impl A {
    fn new() -> Self {
        Self {
            immut: Arc::new(0),
            mutt: 0,
        }
    }

    fn f(&mut self) {
        let immut = self.immut.clone();
        self.mutt = 2;
        println!("{}", immut);
    }
}

fn main() {
    let mut a = A::new();
    a.f();
}