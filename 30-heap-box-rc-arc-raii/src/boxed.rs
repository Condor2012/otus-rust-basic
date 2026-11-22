pub fn boxed_value(value: i32) -> Box<i32> {
    Box::new(value)
}

pub fn boxed_slice(values: &[i32]) -> Box<[i32]> {
    values.into()
}

pub trait Number {
    fn value(&self) -> i32;
}

pub struct HeapNumber(pub i32);

impl Number for HeapNumber {
    fn value(&self) -> i32 {
        self.0
    }
}

pub fn boxed_trait(value: i32) -> Box<dyn Number> {
    Box::new(HeapNumber(value))
}

#[derive(Debug, PartialEq, Eq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn from_slice(values: &[i32]) -> Self {
        values
            .iter()
            .rev()
            .fold(Self::Nil, |tail, value| Self::Cons(*value, Box::new(tail)))
    }

    pub fn sum(&self) -> i32 {
        match self {
            Self::Cons(value, tail) => value + tail.sum(),
            Self::Nil => 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Cons(_, tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_owns_value_and_supports_deref() {
        let value = boxed_value(42);

        assert_eq!(*value, 42);
    }

    #[test]
    fn boxed_slice_has_fixed_length() {
        let values = boxed_slice(&[1, 2, 3]);

        assert_eq!(values.as_ref(), &[1, 2, 3]);
    }

    #[test]
    fn box_can_own_a_trait_object() {
        assert_eq!(boxed_trait(7).value(), 7);
    }

    #[test]
    fn box_makes_recursive_list_finite_sized() {
        let list = List::from_slice(&[1, 2, 3]);

        assert_eq!(list.len(), 3);
        assert_eq!(list.sum(), 6);
        assert_eq!(List::Nil.len(), 0);
    }
}
