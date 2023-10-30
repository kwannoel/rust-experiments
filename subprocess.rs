use tokio;
use tokio_retry;
use std::fs;

async fn main_ok() {
    tokio::spawn(async {
        loop {
            fs::write("foo.txt", "bar").unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    // drop(task);
}

#[tokio::main]
async fn main() {
    let join_handle = tokio::spawn(subtask());
    join_handle.await.unwrap();
}

async fn subtask() {
    let retry_strategy = tokio_retry::strategy::FixedInterval::from_millis(1000);
    let result = tokio_retry::Retry::spawn(retry_strategy, || async  {
        fs::write("foo.txt", "bar").unwrap();
        Err::<(), &str>("error")
    });
    result.await;
    // drop(task);
}