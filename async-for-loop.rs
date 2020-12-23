use std::{thread, time};

use futures::executor::block_on;

fn main() -> () {
    println!("{:?}", block_on(prog_err_ignored()));
}

async fn prog_err_ignored() -> Option<()> {

    let futures = vec![
        f(0, "s0", Some(())),
        f(100_000, "e10_000", None),
        f(10_000, "e5000", None),
        f(1000, "e1000", None)
    ];

    for future in futures.iter() {
        future.await;
    }

    return Some(());
}

async fn f(time: u64, msg: &str, ret: Option<()>) -> Option<()> {
    let ms = time::Duration::from_millis(time);
    thread::sleep(ms);
    println!("{}", msg);
    ret
}
