use std::ops::Add;

pub trait TupleSum2 {
    type Output;

    fn sum_first_two(self) -> Self::Output;
}

pub fn tuple_sum2<T: TupleSum2>(tuple: T) -> T::Output {
    tuple.sum_first_two()
}

impl<A, B> TupleSum2 for (A, B)
where
    A: Copy + Add<B, Output = A>,
    B: Copy,
{
    type Output = A;

    fn sum_first_two(self) -> Self::Output {
        self.0 + self.1
    }
}

macro_rules! impl_tuple_sum2 {
    ($($rest:ident),+) => {
        impl<A, B, $($rest),+> TupleSum2 for (A, B, $($rest),+)
        where
            A: Copy + Add<B, Output = A>,
            B: Copy,
        {
            type Output = A;

            fn sum_first_two(self) -> Self::Output {
                self.0 + self.1
            }
        }
    };
}

impl_tuple_sum2!(C);
impl_tuple_sum2!(C, D);
impl_tuple_sum2!(C, D, E);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_tuples_of_supported_lengths() {
        assert_eq!(tuple_sum2((5, 3)), 8);
        assert_eq!(tuple_sum2((5.0, 3.0, "hello")), 8.0);
        assert_eq!(tuple_sum2((1, 2, 3, 4)), 3);
        assert_eq!(tuple_sum2((10, 20, 30, 40, 50)), 30);
    }
}
