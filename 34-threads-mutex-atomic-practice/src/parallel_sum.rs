use std::thread;

/// Делит вектор пополам и суммирует обе части в отдельных потоках.
pub fn run() -> i32 {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mid = numbers.len() / 2;
    let first_half = numbers[..mid].to_vec();
    let second_half = numbers[mid..].to_vec();

    let handle1 = thread::spawn(move || first_half.iter().sum::<i32>());
    let handle2 = thread::spawn(move || second_half.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn sums_all_numbers() {
        assert_eq!(run(), 55);
    }
}
