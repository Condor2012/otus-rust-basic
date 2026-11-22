pub trait BoundedNumeric: Sized {
    const MIN: Self = Self::ZERO;
    const MAX: Self;
    const ZERO: Self;

    fn is_valid(&self) -> bool
    where
        Self: PartialOrd,
    {
        self >= &Self::MIN && self <= &Self::MAX
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl BoundedNumeric for Celsius {
    const MIN: Self = Celsius(-273.15);
    const MAX: Self = Celsius(1_000_000.0);
    const ZERO: Self = Celsius(0.0);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f32);

impl BoundedNumeric for Percentage {
    const MAX: Self = Percentage(100.0);
    const ZERO: Self = Percentage(0.0);
}

pub fn normalize<T>(value: T) -> T
where
    T: BoundedNumeric + Copy + PartialOrd,
{
    if value < T::MIN {
        T::MIN
    } else if value > T::MAX {
        T::MAX
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associated_constants_define_type_specific_bounds() {
        assert_eq!(normalize(Celsius(25.0)), Celsius(25.0));
        assert_eq!(normalize(Celsius(-300.0)), Celsius(-273.15));
        assert_eq!(normalize(Percentage(50.0)), Percentage(50.0));
        assert_eq!(normalize(Percentage(150.0)), Percentage(100.0));
        assert_eq!(normalize(Percentage(-1.0)), Percentage(0.0));
    }

    #[test]
    fn validity_uses_the_same_bounds() {
        assert!(Celsius(0.0).is_valid());
        assert!(!Celsius(-300.0).is_valid());
        assert!(Percentage(100.0).is_valid());
        assert!(!Percentage(101.0).is_valid());
    }
}
