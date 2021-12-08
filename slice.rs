pub fn main() {
    let a: Vec<u8> = vec![1, 2, 3];
    let b: Vec<u8> = Vec::from(&a[0..3]);
    println!("{:?}", b);
    let c = &a[0..4]; // Panics here, because slice is oob.
    println!("{:?}", c);
}
