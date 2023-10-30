//! 1. Consider a stream in tokio.
//!    If only the first item is polled, will we also iter on the subsequent ones?
//! 2. Consider N streams in tokio.
//!    After they are merged, will we poll the first item of all of them?
use tokio;
use std::fs;

#[tokio::main]
async fn main() {
    println!("Hello, world!")
}
