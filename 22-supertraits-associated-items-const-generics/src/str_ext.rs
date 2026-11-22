use std::fmt::Display;

pub trait ToStringExt: Display {
    fn to_square_str(&self) -> String {
        format!("[{self}]")
    }

    fn to_curly_str(&self) -> String {
        format!("{{{self}}}")
    }
}

impl<T: Display> ToStringExt for T {}

pub trait VecExt<T> {
    fn ends(&self) -> Option<(&T, &T)>;
}

impl<T> VecExt<T> for Vec<T> {
    fn ends(&self) -> Option<(&T, &T)> {
        let first = self.first()?;
        let last = self.last()?;
        Some((first, last))
    }
}

pub trait IntExt {
    fn is_even(&self) -> bool;
}

impl IntExt for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::{IntExt, ToStringExt, VecExt};

    #[test]
    fn display_types_receive_extension_methods() {
        assert_eq!(123_i32.to_square_str(), "[123]");
        assert_eq!(123_u32.to_curly_str(), "{123}");
        assert_eq!(
            Ipv4Addr::new(127, 0, 0, 1).to_square_str(),
            "[127.0.0.1]"
        );
    }

    #[test]
    fn vector_ends_are_borrowed() {
        let values = vec![1, 2, 3];
        assert_eq!(values.ends(), Some((&1, &3)));
        assert_eq!(Vec::<i32>::new().ends(), None);
        assert_eq!(vec![7].ends(), Some((&7, &7)));
    }

    #[test]
    fn integer_extension_works() {
        assert!(42.is_even());
        assert!(!43.is_even());
    }
}
