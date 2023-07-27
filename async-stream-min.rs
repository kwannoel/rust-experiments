#![feature(type_alias_impl_trait)]
#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use anyhow::{anyhow, Result};
use futures::stream::Stream;
use futures_async_stream::{stream, for_await};

type Op = impl Stream<Item = Result<usize>>;
async fn iter() -> Op {
    futures::stream::iter(vec![Ok(1), Err(anyhow!(""))])
}

#[stream(item = Result<usize>)]
async fn transform<S>(stream: S)
where
    S: Stream<Item = Result<usize>> + Unpin,
{
    #[for_await]
    for i in stream {
        yield i;
    }
}

async fn bad() -> Op {
    transform(Box::pin(iter().await))
}

#[allow(dead_code)]
#[stream(item = Result<usize>)]
async fn good() {
    #[for_await]
    for i in transform(Box::pin(iter().await)) {
        yield i;
    }
}

#[tokio::main]
async fn main() {
    let s = bad().await;
    // let s = good();
    #[for_await]
    for i in s {
        println!("{:#?}", i);
    }
}
