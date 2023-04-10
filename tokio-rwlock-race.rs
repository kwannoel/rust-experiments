use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    main_deadlock().await;
    // main_correct().await;
}

async fn main_deadlock() {
    // NOTE: This is demo for deadlock
    let lock = RwLock::new(5);

    // many reader locks can be held at once
    let r1 = lock.read().await;
    let r2 = lock.read().await;
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);

    // We try to acquire a write lock, when it reader locks are held in scope.
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    }
}

async fn main_correct() {
    let lock = RwLock::new(5);

    // many reader locks can be held at once
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // read locks are dropped at this point

    // only one write lock may be held, however
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // write lock is dropped here
}
