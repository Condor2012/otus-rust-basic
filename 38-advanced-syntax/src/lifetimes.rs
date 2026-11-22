pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
}

impl Book<'_> {
    pub fn get_title(&self) -> &str {
        self.title
    }
}

pub const STATIC_TEXT: &'static str = "статическая строка";

#[cfg(test)]
mod tests {
    use super::{Book, STATIC_TEXT, longest};

    #[test]
    fn returned_reference_borrows_from_inputs() {
        let first = String::from("длинная строка");
        let result;
        {
            let second = String::from("короткая");
            result = longest(&first, &second);
            assert_eq!(result, first);
        }
    }

    #[test]
    fn struct_and_static_lifetimes() {
        let book = Book { title: "Rust", author: "Анна" };
        assert_eq!(book.get_title(), "Rust");
        assert_eq!(STATIC_TEXT, "статическая строка");
    }
}
