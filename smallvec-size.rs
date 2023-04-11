use std::mem::size_of;
use smallvec::{SmallVec, smallvec};

fn main() {
    println!("size_of SmallVec u8 * 16: {}", size_of::<SmallVec<[u8; 16]>>());
    println!("size_of SmallVec u8 * 32: {}", size_of::<SmallVec<[u8; 32]>>());
}