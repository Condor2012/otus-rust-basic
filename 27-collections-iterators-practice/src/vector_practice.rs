pub fn filter_and_uppercase(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .filter(|string| string.len() > 3)
        .map(|string| string.to_uppercase())
        .collect()
}

pub fn find_min(numbers: Vec<i32>) -> Option<i32> {
    numbers.into_iter().min()
}

pub fn sum_elements(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().sum()
}

pub fn remove_all_occurrences(mut numbers: Vec<i32>, target: i32) -> Vec<i32> {
    numbers.retain(|number| *number != target);
    numbers
}

pub fn is_palindrome(numbers: Vec<i32>) -> bool {
    numbers.iter().eq(numbers.iter().rev())
}

pub fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .map(|number| number * 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_and_uppercases_strings() {
        let strings = vec!["rust", "is", "awesome", "and", "fast"];

        assert_eq!(filter_and_uppercase(strings), vec!["RUST", "AWESOME", "FAST"]);
    }

    #[test]
    fn finds_minimum_or_none() {
        assert_eq!(find_min(vec![5, 3, 9, 1, 4]), Some(1));
        assert_eq!(find_min(vec![]), None);
    }

    #[test]
    fn sums_all_elements() {
        assert_eq!(sum_elements(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_elements(vec![]), 0);
    }

    #[test]
    fn retains_only_values_not_equal_to_target() {
        assert_eq!(remove_all_occurrences(vec![1, 2, 3, 2, 4, 2, 5], 2), vec![1, 3, 4, 5]);
        assert_eq!(remove_all_occurrences(vec![2, 2], 2), Vec::<i32>::new());
    }

    #[test]
    fn recognizes_palindromes() {
        assert!(is_palindrome(vec![1, 2, 3, 2, 1]));
        assert!(is_palindrome(vec![]));
        assert!(!is_palindrome(vec![1, 2, 3]));
    }

    #[test]
    fn filters_even_numbers_and_doubles_them() {
        assert_eq!(process_numbers(vec![1, 2, 3, 4, 5, 6]), vec![4, 8, 12]);
    }
}
