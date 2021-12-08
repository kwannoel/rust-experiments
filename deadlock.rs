use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let shared = Arc::new(Mutex::new(0));

    let s1 = shared.clone();
    let thread1 = thread::spawn(move || {
        loop {
            let ten_ms = Duration::from_millis(4000);
            thread::sleep(ten_ms);
            match s1.lock() {
                Ok(mut c) => {
                    *c += 1;
                    println!("{}", c);
                    println!("THREAD 1 SUCCESS")
                },
                Err(e) => {
                    println!("THREAD 1 FAIL")
                }
            }
        }
    });

    let s2 = shared.clone();
    let thread2 = thread::spawn(move || {
        loop {
            let ten_ms = Duration::from_millis(5000);
            thread::sleep(ten_ms);
            match s2.lock() {
                Ok(mut c) => {
                    // Introduce some artificial slowness

                    *c += 1;
                    println!("{}", c);
                    println!("THREAD 2 SUCCESS")
                },
                Err(e) => {
                    println!("THREAD 2 FAIL")
                }
            }
        }
    });

    thread2.join();
    thread1.join();
}
