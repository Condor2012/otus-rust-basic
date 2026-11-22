pub fn parity(values: &[i32]) -> Vec<i32> {
    values.iter().map(|value| value % 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn map_does_nothing_until_iterator_is_consumed() {
        let input = [1, 2, 3];
        let evaluations = Cell::new(0);

        let mut parity = input.iter().map(|value| {
            evaluations.set(evaluations.get() + 1);
            value % 2
        });

        assert_eq!(evaluations.get(), 0);
        assert_eq!(parity.next(), Some(1));
        assert_eq!(evaluations.get(), 1);
        assert_eq!(parity.collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(evaluations.get(), 3);
    }

    #[test]
    fn parity_matches_practice_example() {
        assert_eq!(parity(&[1, 2, 3]), vec![1, 0, 1]);
    }
}
