use std::fmt::Display;

pub trait Described: Display {
    const KIND: &'static str = "displayable";

    fn describe(&self) -> String {
        format!("{}: {self}", Self::KIND)
    }
}

impl<T: Display> Described for T {}

pub trait Converter<T> {
    fn convert(&self) -> T;
}

pub struct Celsius(pub f64);

impl Converter<f64> for Celsius {
    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Celsius, Converter, Described};

    #[test]
    fn blanket_impl_uses_default_method() {
        assert_eq!(42.describe(), "displayable: 42");
    }

    #[test]
    fn generic_trait_converts_to_selected_type() {
        assert_eq!(Celsius(100.0).convert(), 212.0);
    }
}
