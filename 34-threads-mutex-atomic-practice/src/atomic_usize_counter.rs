use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Десять потоков по сто раз увеличивают счётчик без мьютекса.
pub fn run() -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
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
    fn counts_one_thousand_increments() {
        assert_eq!(run(), 1_000);
    }
}
