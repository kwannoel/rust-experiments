#![feature(try_blocks)]

use anyhow::Result;

fn some_func() -> Result<usize> {
    let res: Result<usize> = try {
        return Ok(1);
        1
    };
    return Ok(2);
}

fn main() {
    println!("{:#?}", some_func());
}