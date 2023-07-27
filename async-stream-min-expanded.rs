#![feature(prelude_import)]
#![feature(type_alias_impl_trait)]
#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anyhow::{anyhow, Result};
use futures::stream::Stream;
use futures_async_stream::{stream, for_await};
type Op = impl Stream<Item = Result<usize>>;
async fn iter() -> Op {
    futures::stream::iter(
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Ok(1),
                Err(
                    ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(format_args!(""));
                        error
                    }),
                ),
            ]),
        ),
    )
}
fn transform<S>(
    stream: S,
) -> impl ::futures_async_stream::__private::stream::Stream<Item = Result<usize>>
where
    S: Stream<Item = Result<usize>> + Unpin,
{
    ::futures_async_stream::__private::stream::from_generator(static move |
        mut __task_context: ::futures_async_stream::__private::future::ResumeTy,
    | -> () {
        let (): () = {
            {
                let mut __pinned = stream;
                let mut __pinned = unsafe {
                    ::futures_async_stream::__private::Pin::new_unchecked(&mut __pinned)
                };
                loop {
                    let i = {
                        let __poll_result = unsafe {
                            ::futures_async_stream::__private::stream::Stream::poll_next(
                                ::futures_async_stream::__private::Pin::as_mut(
                                    &mut __pinned,
                                ),
                                ::futures_async_stream::__private::future::get_context(
                                    __task_context,
                                ),
                            )
                        };
                        match __poll_result {
                            ::futures_async_stream::__private::Poll::Ready(
                                ::futures_async_stream::__private::Some(e),
                            ) => e,
                            ::futures_async_stream::__private::Poll::Ready(
                                ::futures_async_stream::__private::None,
                            ) => break,
                            ::futures_async_stream::__private::Poll::Pending => {
                                __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
                                continue;
                            }
                        }
                    };
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        i,
                    ));
                }
            }
        };
        #[allow(unreachable_code)]
        {
            return;
            loop {
                __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
            }
        }
    })
}
async fn bad() -> Op {
    transform(Box::pin(iter().await))
}
#[allow(dead_code)]
fn good() -> impl ::futures_async_stream::__private::stream::Stream<
    Item = Result<usize>,
> {
    ::futures_async_stream::__private::stream::from_generator(static move |
        mut __task_context: ::futures_async_stream::__private::future::ResumeTy,
    | -> () {
        let (): () = {
            {
                let mut __pinned = transform(
                    Box::pin({
                        let mut __pinned = iter();
                        let mut __pinned = unsafe {
                            ::futures_async_stream::__private::Pin::new_unchecked(
                                &mut __pinned,
                            )
                        };
                        loop {
                            if let ::futures_async_stream::__private::Poll::Ready(result)
                                = unsafe {
                                    ::futures_async_stream::__private::future::Future::poll(
                                        ::futures_async_stream::__private::Pin::as_mut(
                                            &mut __pinned,
                                        ),
                                        ::futures_async_stream::__private::future::get_context(
                                            __task_context,
                                        ),
                                    )
                                } {
                                break result;
                            }
                            __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
                        }
                    }),
                );
                let mut __pinned = unsafe {
                    ::futures_async_stream::__private::Pin::new_unchecked(&mut __pinned)
                };
                loop {
                    let i = {
                        let __poll_result = unsafe {
                            ::futures_async_stream::__private::stream::Stream::poll_next(
                                ::futures_async_stream::__private::Pin::as_mut(
                                    &mut __pinned,
                                ),
                                ::futures_async_stream::__private::future::get_context(
                                    __task_context,
                                ),
                            )
                        };
                        match __poll_result {
                            ::futures_async_stream::__private::Poll::Ready(
                                ::futures_async_stream::__private::Some(e),
                            ) => e,
                            ::futures_async_stream::__private::Poll::Ready(
                                ::futures_async_stream::__private::None,
                            ) => break,
                            ::futures_async_stream::__private::Poll::Pending => {
                                __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
                                continue;
                            }
                        }
                    };
                    __task_context = (yield ::futures_async_stream::__private::Poll::Ready(
                        i,
                    ));
                }
            }
        };
        #[allow(unreachable_code)]
        {
            return;
            loop {
                __task_context = (yield ::futures_async_stream::__private::Poll::Pending);
            }
        }
    })
}
fn main() {
    let body = async {
        let s = bad().await;
        {
            let mut __pinned = s;
            let mut __pinned = unsafe {
                ::futures_async_stream::__private::Pin::new_unchecked(&mut __pinned)
            };
            loop {
                let i = match ::futures_async_stream::__private::stream::next(
                        &mut __pinned,
                    )
                    .await
                {
                    ::futures_async_stream::__private::Some(e) => e,
                    ::futures_async_stream::__private::None => break,
                };
                {
                    ::std::io::_print(format_args!("{0:#?}\n", i));
                };
            }
        }
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
