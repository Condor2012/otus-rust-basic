pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn classify_number(input: Option<i32>) -> &'static str {
    match input {
        Some(x) if x % 2 == 0 && x > 10 => "Чётное больше 10",
        Some(x) if x.is_negative() => "Отрицательное",
        Some(_) => "Другое число",
        None => "Нет значения",
    }
}

pub fn locate(point: Point) -> &'static str {
    match point {
        Point { x, y } if x == y => "На диагонали",
        Point { x, y } if x.abs() == y.abs() => "На зеркальной диагонали",
        _ => "В другом месте",
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, classify_number, locate};

    #[test]
    fn applies_guards_after_patterns() {
        assert_eq!(classify_number(Some(12)), "Чётное больше 10");
        assert_eq!(classify_number(Some(-4)), "Отрицательное");
        assert_eq!(classify_number(Some(3)), "Другое число");
        assert_eq!(classify_number(None), "Нет значения");
    }

    #[test]
    fn combines_guards_with_destructuring() {
        assert_eq!(locate(Point { x: 2, y: 2 }), "На диагонали");
        assert_eq!(locate(Point { x: 2, y: -2 }), "На зеркальной диагонали");
        assert_eq!(locate(Point { x: 1, y: 3 }), "В другом месте");
    }
}
