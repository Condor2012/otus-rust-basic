#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
}

#[inline]
pub fn square(value: i32) -> i32 {
    value * value
}

#[allow(dead_code)]
fn reserved_for_future_use() {}

#[cfg(test)]
mod tests {
    use super::{User, square};

    #[test]
    fn derive_and_inline_attributes_work() {
        let user = User {
            name: "Анна".into(),
        };
        assert_eq!(user.clone(), user);
        assert_eq!(format!("{user:?}"), "User { name: \"Анна\" }");
        assert_eq!(square(4), 16);
    }
}
