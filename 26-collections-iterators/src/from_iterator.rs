use std::collections::{BTreeSet, HashSet};

pub fn filter_letters(string: &str) -> String {
    string.chars().filter(|character| character.is_alphabetic()).collect()
}

pub fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

pub fn ordered_unique_items<T: Ord>(values: Vec<T>) -> BTreeSet<T> {
    values.into_iter().collect()
}

pub fn collect_results(results: Vec<Result<i32, String>>) -> Result<Vec<i32>, String> {
    results.into_iter().collect()
}

pub fn collect_options(values: Vec<Option<i32>>) -> Option<Vec<i32>> {
    values.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_builds_different_collection_types() {
        assert_eq!(filter_letters("a1-b2!"), "ab");

        let chars = unique_chars("abca");
        assert_eq!(chars.len(), 3);
        assert!(chars.contains(&'a'));

        let ordered = ordered_unique_items(vec![3, 1, 3, 2]);
        assert_eq!(ordered.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn result_collection_stops_at_first_error() {
        assert_eq!(collect_results(vec![Ok(1), Ok(2)]), Ok(vec![1, 2]));
        assert_eq!(
            collect_results(vec![Ok(1), Err("error".to_string()), Ok(3)]),
            Err("error".to_string())
        );
    }

    #[test]
    fn option_collection_requires_all_values() {
        assert_eq!(collect_options(vec![Some(1), Some(2)]), Some(vec![1, 2]));
        assert_eq!(collect_options(vec![Some(1), None, Some(3)]), None);
    }
}
