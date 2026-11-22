use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

/// Четыре потока по десять раз атомарно увеличивают общий счётчик.
pub fn run() -> i32 {
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn counts_every_atomic_increment() {
        assert_eq!(run(), 40);
    }
}
