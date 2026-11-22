use std::collections::{HashMap, HashSet};

pub fn unique_elements(numbers: Vec<i32>) -> Vec<i32> {
    let unique_set: HashSet<_> = numbers.into_iter().collect();
    unique_set.into_iter().collect()
}

pub fn merge_unique(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = vec1.into_iter().collect();
    let set2: HashSet<_> = vec2.into_iter().collect();
    set1.union(&set2).cloned().collect()
}

pub fn first_unique_element(numbers: Vec<i32>) -> Option<i32> {
    let mut frequency_map = HashMap::new();

    for &number in &numbers {
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    for number in numbers {
        if frequency_map[&number] == 1 {
            return Some(number);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut values: Vec<i32>) -> Vec<i32> {
        values.sort_unstable();
        values
    }

    #[test]
    fn removes_duplicates_without_assuming_hashset_order() {
        assert_eq!(sorted(unique_elements(vec![1, 2, 3, 2, 1, 4, 5])), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn merges_vectors_and_removes_duplicates() {
        assert_eq!(sorted(merge_unique(vec![1, 2, 3], vec![3, 4, 5])), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn finds_first_element_with_frequency_one() {
        assert_eq!(first_unique_element(vec![1, 2, 3, 2, 1, 4, 5]), Some(3));
        assert_eq!(first_unique_element(vec![1, 1, 2, 2]), None);
        assert_eq!(first_unique_element(vec![]), None);
    }
}
