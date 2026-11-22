use std::array;

pub fn take5<T, I>(collection: I) -> [T; 5]
where
    T: Default,
    I: IntoIterator<Item = T>,
{
    take_n(collection)
}

pub fn take_n<T, I, const N: usize>(collection: I) -> [T; N]
where
    T: Default,
    I: IntoIterator<Item = T>,
{
    let mut iterator = collection.into_iter();
    array::from_fn(|_| iterator.next().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take5_uses_const_generic_implementation() {
        assert_eq!(take5(vec![1, 2, 3]), [1, 2, 3, 0, 0]);
        assert_eq!(take5(1..10), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn take_n_supports_different_lengths() {
        let ten: [u32; 10] = take_n(vec![1, 2, 3]);
        let nine: [u32; 9] = take_n(vec![1, 2, 3]);
        let floats: [f32; 4] = take_n(vec![1.0, 2.0]);
        let empty: [i32; 0] = take_n(Vec::new());

        assert_eq!(ten, [1, 2, 3, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(nine, [1, 2, 3, 0, 0, 0, 0, 0, 0]);
        assert_eq!(floats, [1.0, 2.0, 0.0, 0.0]);
        assert_eq!(empty, []);
    }
}
