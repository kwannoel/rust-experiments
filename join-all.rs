use futures::executor::block_on;
use futures::future::try_join_all;
use std::{thread, time};
use tokio::task::JoinError;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    println!("{:?}", block_on(test_join_all()));
    //    println!("{:?}", block_on(test_unordered_futures()));
}

async fn test_join_all() -> Result<(), JoinError> {
    let mut tasks: Vec<JoinHandle<()>> = vec![];
    for duration in [1000, 2000, 3000].iter() {
        tasks.push(tokio::spawn(async move { action(*duration) }))
    }

    try_join_all(tasks).await?; // ignored

    Ok(())
}

// async fn err() -> Option<()> {
//     None
// }

fn action(duration: u64) {
    let ms = time::Duration::from_millis(duration);
    thread::sleep(ms);
    println!("{}", duration);
    panic!("error out");
}
