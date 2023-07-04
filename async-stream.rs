#![feature(proc_macro_hygiene, stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]
#![feature(generators)]

use futures::stream::{Stream};
use futures_async_stream::{for_await, stream, try_stream};



#[derive(Debug)]
struct MyError;
type StreamA = impl Stream<Item = Result<i32, MyError>>;

async fn collect(stream: impl Stream<Item = i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    #[for_await]
    for value in stream {
        vec.push(value);
    }
    vec
}

#[try_stream(ok = i32, error=MyError)]
async fn test_generator() {
    for x in 0..100 {
        yield x;
    }
}

fn test_generator_elided() -> StreamA {
    test_generator()
}

#[stream(item = Result<i32, MyError>)]
async fn test_generator_outer(gen: StreamA) {
    #[for_await]
    for value in gen {
        yield value
    }
}

#[stream(item = Result<i32, MyError>)]
async fn test_generator_outer_elided() {
    #[for_await]
    for value in test_generator_elided() {
        yield value
    }
}

#[try_stream(ok=i32, error=MyError)]
async fn test_generator_outer_elided_v2() {
    #[for_await]
    for value in test_generator_elided() {
        let v = value?;
        yield v;
    }
}

#[tokio::main]
async fn main() {
    #[for_await]
    for v in test_generator_outer(test_generator()) {
        println!("{}", v.unwrap());
    }
    test_generator_elided();
    println!("hello world");
}
