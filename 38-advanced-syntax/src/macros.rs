/// Создаёт Vec из произвольного количества выражений.
#[macro_export]
macro_rules! make_vec {
    ($($value:expr),* $(,)?) => {{
        let mut values = Vec::new();
        $(values.push($value);)*
        values
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn accepts_empty_and_variadic_inputs() {
        let empty: Vec<i32> = make_vec!();
        assert_eq!(empty, Vec::<i32>::new());
        assert_eq!(make_vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(make_vec![1 + 1, 2 + 2,], vec![2, 4]);
    }
}
