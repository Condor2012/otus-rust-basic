#![recursion_limit = "32"]

/// Раскрывается в выражение 1 + 3.
#[macro_export]
macro_rules! four {
    () => {
        1 + 3
    };
}

/// Складывает два произвольных выражения.
#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

/// Аналог vec! с повторением выражений и необязательной последней запятой.
#[macro_export]
macro_rules! make_vec {
    ($($element:expr),* $(,)?) => {{
        let mut values = Vec::new();
        $(values.push($element);)*
        values
    }};
}

/// Решение практической задачи hashmap! со слайдов.
#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        ::std::collections::HashMap::from([$(($key, $value)),*])
    };
}

/// Идентификатор передаётся с одинаковым гигиеническим контекстом в обеих позициях.
#[macro_export]
macro_rules! using_a {
    ($a:ident, $expression:expr) => {{
        let $a = 42;
        $expression
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn expands_simple_expressions() {
        assert_eq!(2 * four!(), 8);
        assert_eq!(add!(5, 1 + 3), 9);
    }

    #[test]
    fn supports_repetitions_and_trailing_comma() {
        let empty: Vec<i32> = make_vec![];
        assert!(empty.is_empty());
        assert_eq!(make_vec![1, 2, 3,], vec![1, 2, 3]);
    }

    #[test]
    fn builds_hashmap() {
        let map = hashmap! { 1 => "one", 2 => "two", };
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
    }

    #[test]
    fn identifier_argument_preserves_hygiene() {
        assert_eq!(using_a!(a, a / 10), 4);
    }
}
