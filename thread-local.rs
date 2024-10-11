use std::sync::atomic::AtomicI64;
use std::thread;
thread_local! {
    static THREAD_LOCAL_DATA: AtomicI64 = AtomicI64::new(0);
}

fn increment_thread_local_data(i: usize) {
    THREAD_LOCAL_DATA.with(|data| {
        let data = data.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("Thread tokio:{:?} sys:{:?} local data: {}", tokio::runtime::Handle::try_current().map(|t| t.id()), thread::current().id(), data);
    });
}

#[tokio::main]
async fn main() {
    // Tokio's thread just reuse the underlying pool of OS threads.
    for i in 0..10 {
        tokio::spawn(
            async move {
                increment_thread_local_data(i);
            }
        ).await.unwrap();
    }

    // No tokio runtime here, just an os thread.
    // But wait a second... Why do we get:
    // Thread tokio:Err(TryCurrentError { kind: NoContext }) sys:ThreadId(12) local data: 0
    // Let's look at how tokio maintains the runtime context.
    thread::spawn(
        || {
            increment_thread_local_data(10);
        }
    ).join().unwrap();

    // Direct spawn on tokio runtime.
    increment_thread_local_data(11);
}

// The output will be something like:
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 0
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 1
// Thread tokio:Ok(Id(1)) sys:ThreadId(10) local data: 0
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 2
// Thread tokio:Ok(Id(1)) sys:ThreadId(10) local data: 1
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 3
// Thread tokio:Ok(Id(1)) sys:ThreadId(10) local data: 2
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 4
// Thread tokio:Ok(Id(1)) sys:ThreadId(10) local data: 3
// Thread tokio:Ok(Id(1)) sys:ThreadId(11) local data: 5
// Thread tokio:Err(TryCurrentError { kind: NoContext }) sys:ThreadId(12) local data: 0
// Thread tokio:Ok(Id(1)) sys:ThreadId(1) local data: 0
