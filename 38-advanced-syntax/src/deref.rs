use std::ops::{Deref, DerefMut};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn text_len(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use super::{MyBox, text_len};

    #[test]
    fn supports_deref_coercion_and_mutation() {
        let mut value = MyBox::new(String::from("Привет"));
        assert_eq!(text_len(&value), 12);
        value.push('!');
        assert_eq!(&*value, "Привет!");
    }
}
