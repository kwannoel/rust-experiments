use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let print = |s| { println!("{}", s) };
    let print_hdl = Arc::new(Mutex::new(PrintHandle { print }));

    let print_hdl_1 = print_hdl.clone();
    let t1 = thread::spawn(move || {
        loop {
            let print_lock = print_hdl_1.lock().unwrap();
            (print_lock.print)("A".to_owned());
            drop(print_lock);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let print_hdl_2 = print_hdl.clone();
    let t2 = thread::spawn(move || {
        loop {
            let print_lock = print_hdl_2.lock().unwrap();
            (print_lock.print)("B".to_owned());
            drop(print_lock);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let ts = vec![t1, t2];
    for t in ts {
        t.join().unwrap();
    }
}

struct PrintHandle {
    print: fn(x: String)
}
