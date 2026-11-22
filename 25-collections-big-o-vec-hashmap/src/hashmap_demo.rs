use std::collections::HashMap;

pub fn score_operations() -> HashMap<&'static str, i32> {
    let mut scores = HashMap::with_capacity(8);

    scores.insert("Alice", 10);
    scores.insert("Bob", 12);

    // insert заменяет значение существующего ключа.
    assert_eq!(scores.insert("Alice", 11), Some(10));

    // entry не перезаписывает существующее значение.
    scores.entry("Bob").or_insert(99);
    scores.entry("Carol").or_insert_with(|| 8);

    if let Some(score) = scores.get_mut("Bob") {
        *score += 1;
    }

    assert_eq!(scores.get("Alice"), Some(&11));
    assert!(scores.contains_key("Carol"));
    assert_eq!(scores.remove_entry("Alice"), Some(("Alice", 11)));

    scores.reserve(16);
    scores.shrink_to_fit();
    scores
}

pub fn sorted_scores(scores: &HashMap<&'static str, i32>) -> Vec<(&'static str, i32)> {
    let mut result = scores
        .iter()
        .map(|(name, score)| (*name, *score))
        .collect::<Vec<_>>();
    result.sort_unstable_by_key(|(name, _)| *name);
    result
}

pub fn clear_scores(scores: &mut HashMap<&'static str, i32>) {
    scores.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_entry_get_mut_and_remove_work_together() {
        let scores = score_operations();

        assert_eq!(scores.len(), 2);
        assert!(!scores.is_empty());
        assert_eq!(scores.get("Alice"), None);
        assert_eq!(scores.get("Bob"), Some(&13));
        assert_eq!(scores.get("Carol"), Some(&8));
        assert!(scores.capacity() >= scores.len());
        assert_eq!(sorted_scores(&scores), vec![("Bob", 13), ("Carol", 8)]);
    }

    #[test]
    fn clear_empties_the_map() {
        let mut scores = score_operations();

        clear_scores(&mut scores);

        assert!(scores.is_empty());
        assert_eq!(scores.len(), 0);
    }
}
