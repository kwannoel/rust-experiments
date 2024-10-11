use tokio;
use tokio::runtime::Handle;

#[tokio::main]
async fn main() {
    #[derive(Debug)]
    struct A {}
    impl Drop for A {
        fn drop(&mut self) {
            let handle = Handle::current();
            handle.spawn(async move {
                println!("Dropped A");
            });
        }
    }

    let a = A {};
    let handle = tokio::spawn(async move {
        println!("a: {:?}", a);
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    });

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    handle.abort();
}