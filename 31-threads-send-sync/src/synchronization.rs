use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex, RwLock};
use std::thread;

pub fn atomic_counter() -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

pub fn mutex_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

pub fn rwlock_example() -> Vec<i32> {
    let values = RwLock::new(vec![1, 2]);
    values.write().unwrap().push(3);
    values.read().unwrap().clone()
}

pub fn barrier_example() {
    let barrier = Arc::new(Barrier::new(2));
    let thread_barrier = Arc::clone(&barrier);

    let handle = thread::spawn(move || {
        thread_barrier.wait();
    });

    barrier.wait();
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_and_mutex_preserve_all_increments() {
        assert_eq!(atomic_counter(), 400);
        assert_eq!(mutex_counter(), 400);
    }

    #[test]
    fn rwlock_provides_exclusive_write_and_shared_read() {
        assert_eq!(rwlock_example(), vec![1, 2, 3]);
    }

    #[test]
    fn barrier_lets_both_threads_reach_common_point() {
        barrier_example();
    }
}
