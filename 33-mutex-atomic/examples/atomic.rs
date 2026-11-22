use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    counter.store(100_000, Ordering::Relaxed);

    for _i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let _ = counter.try_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v + 1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.load(Ordering::Relaxed));
}
