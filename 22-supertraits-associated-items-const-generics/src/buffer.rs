pub struct Buffer<const N: usize> {
    pub data: [u8; N],
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self { data: [0; N] }
    }
}

impl<const N: usize> Buffer<N> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_length_is_part_of_type() {
        let small = Buffer::<10>::default();
        let large = Buffer::<20>::default();
        let empty = Buffer::<0>::default();

        assert_eq!(small.len(), 10);
        assert_eq!(large.len(), 20);
        assert!(empty.is_empty());
    }
}
