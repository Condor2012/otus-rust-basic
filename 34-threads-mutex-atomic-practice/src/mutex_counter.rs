use std::sync::{Arc, Mutex};
use std::thread;

/// Четыре потока по пять раз увеличивают общий счётчик.
pub fn run() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn counts_every_increment() {
        assert_eq!(run(), 20);
    }
}
