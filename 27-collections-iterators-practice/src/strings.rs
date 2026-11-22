use std::collections::HashMap;

pub fn group_by_length(strings: Vec<String>) -> HashMap<usize, Vec<String>> {
    let mut groups = HashMap::new();

    for string in strings {
        groups.entry(string.len()).or_insert_with(Vec::new).push(string);
    }

    groups
}

pub fn find_long_string(words: Vec<String>) -> String {
    words
        .into_iter()
        .find(|string| string.len() > 5)
        .unwrap_or_else(|| "Not found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_strings_by_byte_length() {
        let strings = vec![
            "apple".to_string(),
            "banana".to_string(),
            "kiwi".to_string(),
            "mango".to_string(),
        ];

        let groups = group_by_length(strings);

        assert_eq!(groups.get(&5), Some(&vec!["apple".to_string(), "mango".to_string()]));
        assert_eq!(groups.get(&6), Some(&vec!["banana".to_string()]));
        assert_eq!(groups.get(&4), Some(&vec!["kiwi".to_string()]));
    }

    #[test]
    fn finds_first_string_longer_than_five_bytes() {
        let words = vec!["cat".to_string(), "tiger".to_string(), "elephant".to_string()];

        assert_eq!(find_long_string(words), "elephant");
        assert_eq!(
            find_long_string(vec!["a".to_string(), "bc".to_string(), "def".to_string()]),
            "Not found"
        );
    }
}
