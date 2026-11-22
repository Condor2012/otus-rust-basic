pub fn nested_success(value: Option<Result<i32, String>>) -> Option<i32> {
    if let Some(Ok(number)) = value {
        Some(number)
    } else {
        None
    }
}

pub fn greater_than_ten(value: Option<i32>) -> Option<i32> {
    if let Some(number) = value && number > 10 {
        Some(number)
    } else {
        None
    }
}

pub fn collect_with_while_let(values: Vec<i32>) -> Vec<i32> {
    let mut iter = values.into_iter();
    let mut result = Vec::new();
    while let Some(value) = iter.next() {
        result.push(value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{collect_with_while_let, greater_than_ten, nested_success};

    #[test]
    fn handles_nested_pattern_and_let_chain() {
        assert_eq!(nested_success(Some(Ok(5))), Some(5));
        assert_eq!(nested_success(Some(Err("нет".into()))), None);
        assert_eq!(greater_than_ten(Some(11)), Some(11));
        assert_eq!(greater_than_ten(Some(10)), None);
    }

    #[test]
    fn consumes_iterator_with_while_let() {
        assert_eq!(collect_with_while_let(vec![1, 2, 3]), vec![1, 2, 3]);
    }
}
