#[allow(clippy::vec_init_then_push)]
pub fn create_with_push() -> Vec<i32> {
    let mut values = Vec::new();
    values.push(1);
    values.push(2);
    values.push(3);
    values
}

pub fn create_with_macro() -> Vec<i32> {
    vec![1, 2, 3]
}

pub fn capacity_survives_clear() -> (usize, usize, usize) {
    let mut values = Vec::with_capacity(1024);
    values.push(1);
    values.push(2);
    values.push(3);

    let capacity_before = values.capacity();
    values.clear();

    (capacity_before, values.len(), values.capacity())
}

pub fn edit_and_sort() -> Vec<i32> {
    let mut values = create_with_macro();

    values.push(4);
    assert_eq!(values.pop(), Some(4));

    values.insert(1, 10);
    assert_eq!(values.remove(0), 1);
    assert_eq!(values.swap_remove(0), 10);

    values.sort_unstable();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_methods_produce_equal_vectors() {
        assert_eq!(create_with_push(), create_with_macro());
    }

    #[test]
    fn clear_removes_elements_but_keeps_allocation() {
        let (capacity_before, len_after, capacity_after) = capacity_survives_clear();

        assert!(capacity_before >= 1024);
        assert_eq!(len_after, 0);
        assert_eq!(capacity_after, capacity_before);
    }

    #[test]
    fn vector_supports_stack_and_positional_operations() {
        assert_eq!(edit_and_sort(), vec![2, 3]);
    }
}
