use tokio;
use std::fs;

#[tokio::main]
async fn main() {
    let _ = some_function().await;
    println!("Hello, world!")
}

async fn some_function() -> usize {
    tokio::spawn(async move {
        loop {
            println!("hi")
        }
    });
    return 1;
}