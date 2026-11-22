use std::fmt::Debug;

pub trait MaybeFrom<T>: Sized {
    type Error: Debug;

    fn maybe_from(value: T) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatToIntErr;

impl MaybeFrom<f32> for i32 {
    type Error = FloatToIntErr;

    fn maybe_from(value: f32) -> Result<Self, Self::Error> {
        let widened = f64::from(value);
        let in_range = widened >= f64::from(i32::MIN) && widened <= f64::from(i32::MAX);

        if value.is_finite() && in_range && value.fract() == 0.0 {
            Ok(value as i32)
        } else {
            Err(FloatToIntErr)
        }
    }
}

pub fn handle_iterator<T>(iterator: T)
where
    T: Iterator,
    T::Item: Copy + PartialEq,
{
    let mut values: Vec<T::Item> = iterator.collect();
    if let Some(first) = values.first().copied() {
        values.retain(|value| *value == first);
    }
    println!("Элементов, равных первому: {}", values.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_from_accepts_exact_integer_floats() {
        assert_eq!(i32::maybe_from(42.0), Ok(42));
        assert_eq!(i32::maybe_from(-7.0), Ok(-7));
    }

    #[test]
    fn maybe_from_rejects_invalid_conversions() {
        assert_eq!(i32::maybe_from(42.5), Err(FloatToIntErr));
        assert_eq!(i32::maybe_from(f32::NAN), Err(FloatToIntErr));
        assert_eq!(i32::maybe_from(f32::INFINITY), Err(FloatToIntErr));
        assert_eq!(i32::maybe_from(3.0e30), Err(FloatToIntErr));
    }

    #[test]
    fn iterator_associated_item_can_be_constrained() {
        handle_iterator(vec![1, 2, 1].into_iter());
    }
}
