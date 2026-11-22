use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

pub fn add<T, U>(a: T, b: U) -> T
where
    T: Add<U, Output = T>,
{
    a + b
}

pub fn largest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    let mut result = values.first()?;

    for value in &values[1..] {
        if value > result {
            result = value;
        }
    }

    Some(result)
}

pub fn print_display<T: Display>(value: T) {
    println!("Display: {value}");
}

pub trait IsNearlyZero {
    fn is_nearly_zero(&self) -> bool;
}

impl IsNearlyZero for i32 {
    fn is_nearly_zero(&self) -> bool {
        *self == 0
    }
}

impl IsNearlyZero for f32 {
    fn is_nearly_zero(&self) -> bool {
        self.abs() < 0.000001
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl IsNearlyZero for Point {
    fn is_nearly_zero(&self) -> bool {
        self.x.is_nearly_zero() && self.y.is_nearly_zero()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn safe_div<T>(a: T, b: T) -> Option<T>
where
    T: Div<Output = T> + IsNearlyZero,
{
    if b.is_nearly_zero() {
        None
    } else {
        Some(a / b)
    }
}

pub trait Klone {
    fn klone(&self) -> Self;
}

impl Klone for i32 {
    fn klone(&self) -> Self {
        *self
    }
}

/// Обобщённый поиск, аналогичный сигнатуре `HashMap::get`.
pub fn get_value<'a, K, V, Q>(map: &'a HashMap<K, V>, key: &Q) -> Option<&'a V>
where
    K: Borrow<Q> + Hash + Eq,
    Q: Hash + Eq + ?Sized,
{
    map.get(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_pair_works_with_different_types() {
        assert_eq!(
            Pair::new(1, 2),
            Pair {
                first: 1,
                second: 2
            }
        );
        assert_eq!(
            Pair::new("left", "right"),
            Pair {
                first: "left",
                second: "right",
            }
        );
    }

    #[test]
    fn add_is_statically_specialized() {
        assert_eq!(add(2_i32, 3), 5);
        assert_eq!(add("Hello, ".to_owned(), "world"), "Hello, world");
    }

    #[test]
    fn largest_does_not_require_copy() {
        let values = vec!["a".to_owned(), "c".to_owned(), "b".to_owned()];
        assert_eq!(largest(&values).map(String::as_str), Some("c"));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn nearly_zero_supports_numbers_and_point() {
        assert!(0_i32.is_nearly_zero());
        assert!(!1_i32.is_nearly_zero());
        assert!(0.0000001_f32.is_nearly_zero());
        assert!(Point::default().is_nearly_zero());
        assert!(!Point { x: 1.0, y: 0.0 }.is_nearly_zero());
    }

    #[test]
    fn safe_div_uses_trait_bounds() {
        assert_eq!(safe_div(10_i32, 2), Some(5));
        assert_eq!(safe_div(10_i32, 0), None);
        assert_eq!(safe_div(10.0_f32, 4.0), Some(2.5));
        assert_eq!(safe_div(10.0_f32, 0.0), None);
    }

    #[test]
    fn point_implements_default_and_add() {
        let a = Point { x: 1.0, y: 2.0 };
        let b = Point { x: 3.0, y: 4.0 };
        assert_eq!(a + b, Point { x: 4.0, y: 6.0 });
        assert_eq!(Point::default(), Point { x: 0.0, y: 0.0 });
        assert_eq!(42_i32.klone(), 42);
    }

    #[test]
    fn hash_map_can_be_queried_by_borrowed_str() {
        let mut map = HashMap::new();
        map.insert("answer".to_owned(), 42);

        assert_eq!(get_value(&map, "answer"), Some(&42));
        assert_eq!(get_value(&map, "missing"), None);
    }
}
