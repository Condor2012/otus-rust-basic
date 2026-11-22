use std::collections::HashMap;

pub fn count_frequencies(numbers: Vec<i32>) -> HashMap<i32, usize> {
    let mut frequency_map = HashMap::new();

    for number in numbers {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    frequency_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_every_number() {
        let frequencies = count_frequencies(vec![1, 2, 3, 2, 1, 3, 4, 5, 4, 1]);

        assert_eq!(frequencies, HashMap::from([(1, 3), (2, 2), (3, 2), (4, 2), (5, 1)]));
    }

    #[test]
    fn empty_vector_returns_empty_map() {
        assert!(count_frequencies(vec![]).is_empty());
    }
}
