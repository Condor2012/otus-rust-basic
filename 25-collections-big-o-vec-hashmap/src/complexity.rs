/// O(1): время доступа не зависит от длины среза.
pub fn first(values: &[i32]) -> Option<i32> {
    values.first().copied()
}

/// O(log n): бинарный поиск требует отсортированного среза.
pub fn binary_search(values: &[i32], target: i32) -> Option<usize> {
    values.binary_search(&target).ok()
}

/// O(n): в худшем случае необходимо проверить каждый элемент.
pub fn linear_search(values: &[i32], target: i32) -> Option<usize> {
    values.iter().position(|value| *value == target)
}

/// O(n log n): сортировка копии не изменяет исходный срез.
pub fn sorted_copy(values: &[i32]) -> Vec<i32> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result
}

/// O(n²): вложенные циклы формируют все упорядоченные пары.
pub fn all_pairs(values: &[i32]) -> Vec<(i32, i32)> {
    let mut pairs = Vec::with_capacity(values.len() * values.len());

    for left in values {
        for right in values {
            pairs.push((*left, *right));
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demonstrates_constant_logarithmic_and_linear_operations() {
        let values = [1, 3, 5, 7];

        assert_eq!(first(&values), Some(1));
        assert_eq!(first(&[]), None);
        assert_eq!(binary_search(&values, 5), Some(2));
        assert_eq!(binary_search(&values, 6), None);
        assert_eq!(linear_search(&values, 7), Some(3));
        assert_eq!(linear_search(&values, 8), None);
    }

    #[test]
    fn sorting_returns_a_new_vector() {
        let values = [3, 1, 2];

        assert_eq!(sorted_copy(&values), vec![1, 2, 3]);
        assert_eq!(values, [3, 1, 2]);
    }

    #[test]
    fn nested_loops_produce_n_squared_pairs() {
        assert_eq!(
            all_pairs(&[1, 2]),
            vec![(1, 1), (1, 2), (2, 1), (2, 2)]
        );
        assert_eq!(all_pairs(&[1, 2, 3]).len(), 9);
    }
}
