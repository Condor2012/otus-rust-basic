use std::thread;

/// Считает суммы двух частей массива в потоках и складывает результаты.
pub fn run() -> i32 {
    let numbers = [1, 2, 3, 4, 5];

    let handle1 = thread::spawn(move || numbers[0..3].iter().sum::<i32>());
    let handle2 = thread::spawn(move || numbers[3..5].iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn returns_total_sum() {
        assert_eq!(run(), 15);
    }
}
