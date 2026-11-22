use std::ops::{Add, Mul};

pub trait Num: Copy + Add<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
}

macro_rules! impl_num {
    ($($type:ty),+) => {
        $(
            impl Num for $type {
                fn zero() -> Self {
                    0 as $type
                }
            }
        )+
    };
}

impl_num!(i32, i64, u32, u64, usize, f32, f64);

pub trait FloatNum: Num {
    fn sqrt(self) -> Self;
}

impl FloatNum for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl FloatNum for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T> Vector<T, 1> {
    pub fn new1(x: T) -> Self {
        Self { data: [x] }
    }
}

impl<T> Vector<T, 2> {
    pub fn new2(x: T, y: T) -> Self {
        Self { data: [x, y] }
    }
}

impl<T> Vector<T, 3> {
    pub fn new3(x: T, y: T, z: T) -> Self {
        Self { data: [x, y, z] }
    }
}

impl<T: FloatNum, const N: usize> Vector<T, N> {
    pub fn len(&self) -> T {
        self.data
            .iter()
            .copied()
            .map(|value| value * value)
            .fold(T::zero(), |sum, value| sum + value)
            .sqrt()
    }
}

impl<T: Num, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: std::array::from_fn(|index| self.data[index] + rhs.data[index]),
        }
    }
}

impl<T: Num, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            data: self.data.map(|value| value * rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_vectors_from_arrays_and_constructors() {
        assert_eq!(Vector::from([1, 2, 3]), Vector::new3(1, 2, 3));
        assert_eq!(Vector::new1(5).data, [5]);
        assert_eq!(Vector::new2(1.0, 2.0).data, [1.0, 2.0]);
    }

    #[test]
    fn calculates_length_for_floating_point_vectors() {
        assert_eq!(Vector::new2(3.0_f64, 4.0).len(), 5.0);
        assert_eq!(Vector::new3(2.0_f32, 3.0, 6.0).len(), 7.0);
    }

    #[test]
    fn adds_vectors_component_wise() {
        let sum = Vector::new3(1, 2, 3) + Vector::new3(10, 20, 30);

        assert_eq!(sum, Vector::new3(11, 22, 33));
    }

    #[test]
    fn multiplies_vector_by_scalar() {
        assert_eq!(Vector::new2(1.5, -2.0) * 2.0, Vector::new2(3.0, -4.0));
        assert_eq!(Vector::new3(1, 2, 3) * 3, Vector::new3(3, 6, 9));
    }
}
