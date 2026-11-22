use std::collections::HashMap;

pub fn copy_from_slice(source: &[i32]) -> Vec<i32> {
    let mut destination = vec![0; source.len()];
    destination.copy_from_slice(source);
    destination
}

pub fn character_frequencies(text: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for character in text.chars() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }

    map
}

pub fn planet_distances() -> HashMap<String, f64> {
    let mut distance = HashMap::from([
        ("Mercury".to_string(), 0.4),
        ("Venus".to_string(), 0.7),
        ("Earth".to_string(), 1.0),
        ("Mars".to_string(), 1.5),
    ]);
    distance.insert("Jupiter".to_string(), 5.2);
    distance
}

pub fn earth_distance(distance: &HashMap<String, f64>) -> Option<f64> {
    // HashMap<String, V> позволяет искать по &str благодаря Borrow<str>.
    distance.get("Earth").copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_from_slice_replaces_destination_contents() {
        assert_eq!(copy_from_slice(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(copy_from_slice(&[]), Vec::<i32>::new());
    }

    #[test]
    fn entry_counts_characters_without_double_lookup() {
        let frequencies = character_frequencies("Let's count character in this string");

        assert_eq!(frequencies.get(&'t'), Some(&5));
        assert_eq!(frequencies.get(&'z'), None);
        assert_eq!(frequencies.get(&' '), Some(&5));
    }

    #[test]
    fn owned_string_keys_support_borrowed_lookup() {
        let distances = planet_distances();

        assert_eq!(earth_distance(&distances), Some(1.0));
        assert_eq!(distances.get("Jupiter"), Some(&5.2));
        assert_eq!(distances.len(), 5);
    }
}
