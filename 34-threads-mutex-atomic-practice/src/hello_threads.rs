use std::thread;

/// Создаёт пять потоков и дожидается завершения каждого.
pub fn run() -> Vec<String> {
    let mut handles = vec![];

    for i in 1..=5 {
        let handle = thread::spawn(move || format!("Привет из потока {i}"));
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
    fn greets_from_all_five_threads() {
        assert_eq!(
            run(),
            (1..=5)
                .map(|i| format!("Привет из потока {i}"))
                .collect::<Vec<_>>()
        );
    }
}
