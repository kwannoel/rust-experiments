use futures::future::{join_all, try_join_all};
use futures::executor::block_on;

fn main() -> () {
    println!("{:?}", block_on(prog_err_ignored()));
    println!("{:?}", block_on(prog_err_caught()));
}

async fn prog_err_ignored() -> Result<(), String> {
    let futures = vec![err(), err(), err()];
    join_all(futures).await; // ignored

    return Ok(());
}

async fn prog_err_caught() -> Result<(), String> {
    let futures = vec![err(), err(), err()];
    try_join_all(futures).await?; // ignored

    return Ok(());
}

async fn err() -> Result<(), String> {
    Err("Err".to_string())
}
