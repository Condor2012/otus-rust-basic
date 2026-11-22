use std::sync::Arc;
use std::thread;

/// Передаёт общее неизменяемое значение трём потокам через Arc.
pub fn run() -> Vec<(usize, i32)> {
    let counter = Arc::new(42);
    let mut handles = vec![];

    for i in 1..=3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || (i, *counter_clone));
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn all_threads_read_the_same_value() {
        assert_eq!(run(), vec![(1, 42), (2, 42), (3, 42)]);
    }
}
