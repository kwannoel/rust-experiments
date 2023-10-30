#[derive(Debug)]
struct X {
    pub a: Option<usize>,
    pub b: Option<usize>,
}

impl X {
    fn get_a(&self) -> &Option<usize> {
        &self.a
    }
}

#[derive(Debug)]
struct Y<'a> {
    a: &'a Option<usize>,
}

fn main() {
    let mut x = X { a: Some(3), b: Some(4)};
    let a = &x.a;

    println!("{:p}", a);
    let a = &x.a;
    println!("{:p}", a);
    let a = x.get_a();
    println!("{:p}", a);
    let y = Y { a };
    // x.b = Some(2);
    println!("{:#?}", x);
    println!("{:#?}", y);
}

fn main2() {
    let mut x = X { a: Some(3), b: Some(4)};
    // let a = &x.a;
    let a = &x.get_a();
    let y = Y { a };
    // x.b = Some(2);
    println!("{:#?}", x);
    println!("{:#?}", y);
}


fn main3() {
    let mut x = X { a: Some(3), b: Some(4)};
    // let a = &x.a;
    //let a = &(*x.get_a());
    let a = x.get_a();
    let y = Y { a };
    // x.b = Some(2);
    println!("{:#?}", x);
    println!("{:#?}", y);
}

fn two_mut_refs_direct() {
    let mut x = X { a: Some(3), b: Some(4)};
    let a = &mut x.a;
    let b = &mut x.b;
    *a = Some(2);
    *b = Some(3);
    println!("{:#?}", a);
}

fn mut_x_mod(x: &mut X) {
    let a = &mut x.a;
    let b = &mut x.b;
    *a = Some(2);
    *b = Some(3);
}