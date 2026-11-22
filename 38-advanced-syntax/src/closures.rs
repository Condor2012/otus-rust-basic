pub fn apply<F>(function: F) -> i32
where
    F: Fn(i32) -> i32,
{
    function(10)
}

pub fn increment_three_times() -> i32 {
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    increment();
    count
}

pub fn move_into_closure() -> Vec<i32> {
    let data = vec![1, 2, 3];
    let closure = move || data;
    closure()
}

#[cfg(test)]
mod tests {
    use super::{apply, increment_three_times, move_into_closure};

    #[test]
    fn demonstrates_capture_modes() {
        let multiplier = 3;
        assert_eq!(apply(|x| x * multiplier), 30);
        assert_eq!(increment_three_times(), 3);
        assert_eq!(move_into_closure(), vec![1, 2, 3]);
    }
}
