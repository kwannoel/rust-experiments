#![feature(try_blocks)]

use anyhow::Result;

// This function will return early with Ok(1).
fn some_func() -> Result<usize> {
    let res: Result<usize> = try {
        return Ok(1);
        1
    };
    return Ok(2);
}

// Prints 1.
fn main() {
    println!("{:#?}", some_func());
}