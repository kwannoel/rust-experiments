use std::{thread, time};
use futures::executor::block_on;
use futures::stream::FuturesUnordered;
use futures::Stream;
use futures::task::{Context, Poll};
use core::pin::Pin;

#[tokio::main]
async fn main() -> () {
    println!("{:?}", block_on(test_futures_unordered()));
}

async fn test_futures_unordered(cx: &mut Context<'_>) -> Option<()> {
    let mut tasks = FuturesUnordered::new();
    for duration in [3000, 2000, 1000].iter() {
        tasks.push(async move {
            action(*duration)
        })
    }

    let mut ret = Some(());
    // https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
    while let Poll::Ready(v) = Pin::new(&mut tasks).poll_next(cx) {
        if v == None {
            return None // shortcircuit
        }
    }

    return Some(());
}

// async fn my_join_all(tasks: FuturesUnordered<Option<()>>) -> Option<()> {
//     let mut ret = Some(());
//     while let Poll::Ready(v) = tasks.poll_next {
//         if v == None {
//             return None // shortcircuit
//         }
//     }

//     return ret;
// }

fn action(duration: u64) -> Option<()> {
    let ms = time::Duration::from_millis(duration);
    thread::sleep(ms);
    println!("{}", duration);
    None
}
