pub fn manual_next() -> [Option<i32>; 4] {
    let values = vec![1, 2, 3];
    let mut iterator = values.into_iter();

    [
        iterator.next(),
        iterator.next(),
        iterator.next(),
        iterator.next(),
    ]
}

pub fn even_doubles() -> Vec<i32> {
    (0..10).map(|value| value * 2).filter(|value| value % 2 == 0).collect()
}

pub fn first_five_from_infinite_range() -> Vec<i32> {
    (0..).take(5).collect()
}

pub fn enumerate_skip_take() -> Vec<(usize, i32)> {
    (0..10)
        .step_by(2)
        .enumerate()
        .skip(1)
        .take(3)
        .collect()
}

pub fn reverse_and_nth() -> (Option<i32>, Vec<i32>) {
    let third = (0..5).nth(2);
    let reversed = (0..5).rev().collect();
    (third, reversed)
}

pub fn consumers(values: &[i32]) -> (Option<i32>, Option<usize>, i32, Option<i32>, usize, Option<i32>, Option<i32>) {
    (
        values.iter().copied().find(|value| *value > 2),
        values.iter().position(|value| *value == 3),
        values.iter().sum(),
        values.iter().last().copied(),
        values.len(),
        values.iter().max().copied(),
        values.iter().min().copied(),
    )
}

pub fn borrowed_sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn double_in_place(values: &mut [i32]) {
    for value in values.iter_mut() {
        *value *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_eventually_returns_none() {
        assert_eq!(manual_next(), [Some(1), Some(2), Some(3), None]);
    }

    #[test]
    fn adapters_are_composable() {
        assert_eq!(even_doubles(), vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
        assert_eq!(first_five_from_infinite_range(), vec![0, 1, 2, 3, 4]);
        assert_eq!(enumerate_skip_take(), vec![(1, 2), (2, 4), (3, 6)]);
        assert_eq!(reverse_and_nth(), (Some(2), vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn consumers_compute_final_values() {
        assert_eq!(consumers(&[1, 2, 3, 4]), (Some(3), Some(2), 10, Some(4), 4, Some(4), Some(1)));
    }

    #[test]
    fn iter_and_iter_mut_borrow_instead_of_consuming() {
        let mut values = vec![1, 2, 3];

        assert_eq!(borrowed_sum(&values), 6);
        double_in_place(&mut values);
        assert_eq!(values, vec![2, 4, 6]);
    }
}
