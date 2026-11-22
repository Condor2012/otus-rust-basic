#[derive(Debug, Default)]
pub struct MyType {
    pub items: Vec<String>,
}

impl MyType {
    pub fn custom_iter(&self) -> MyTypeIterator<'_> {
        MyTypeIterator {
            index: 0,
            items: &self.items,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}

pub struct MyTypeIterator<'a> {
    index: usize,
    items: &'a [String],
}

impl<'a> Iterator for MyTypeIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

pub fn collect_with_into_iterator(values: Vec<i32>) -> Vec<i32> {
    let mut iterator = IntoIterator::into_iter(values);
    let mut result = Vec::new();

    loop {
        match iterator.next() {
            Some(value) => result.push(value),
            None => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value() -> MyType {
        MyType {
            items: vec!["one".to_string(), "two".to_string()],
        }
    }

    #[test]
    fn custom_iterator_returns_borrowed_items() {
        let value = value();
        let mut iterator = value.custom_iter();

        assert_eq!(iterator.next().map(String::as_str), Some("one"));
        assert_eq!(iterator.next().map(String::as_str), Some("two"));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn standard_slice_iterator_can_replace_custom_one() {
        let value = value();

        assert_eq!(value.iter().map(String::as_str).collect::<Vec<_>>(), vec!["one", "two"]);
    }

    #[test]
    fn for_loop_desugars_to_into_iterator_and_next() {
        assert_eq!(collect_with_into_iterator(vec![1, 2, 3]), vec![1, 2, 3]);
    }
}
