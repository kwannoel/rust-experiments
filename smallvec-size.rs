use smallvec::SmallVec;
use std::mem::size_of;
use tinyvec::TinyVec;

fn main() {
    println!(
        "size_of TinyVec u8 * 16: {}",
        size_of::<TinyVec<[u8; 16]>>()
    );
    println!(
        "size_of TinyVec u8 * 32: {}",
        size_of::<TinyVec<[u8; 16]>>()
    );
    println!(
        "size_of SmallVec u8 * 16: {}",
        size_of::<SmallVec<[u8; 16]>>()
    );
    println!(
        "size_of SmallVec u8 * 32: {}",
        size_of::<SmallVec<[u8; 32]>>()
    );
    println!("size_of Option<usize>: {}", size_of::<Option<usize>>());
    println!("size_of usize: {}", size_of::<usize>());
}
