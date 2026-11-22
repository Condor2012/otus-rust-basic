use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Складывает два значения только если оба присутствуют.
pub fn add(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    let sum = a? + b?;
    Some(sum)
}

/// Частичная функция: обратное значение существует не для каждого аргумента.
pub fn reciprocal(value: f64) -> Option<f64> {
    if value == 0.0 {
        None
    } else {
        Some(1.0 / value)
    }
}

pub struct User {
    pub name: String,
    pub nickname: Option<String>,
}

impl User {
    pub fn new(name: &str, nickname: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            nickname: nickname.map(str::to_owned),
        }
    }

    pub fn display_name(&self) -> &str {
        self.nickname.as_deref().unwrap_or(&self.name)
    }

    pub fn take_nickname(&mut self) -> Option<String> {
        self.nickname.take()
    }
}

#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    InvalidNumber(ParseIntError),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "деление на ноль"),
            MathError::InvalidNumber(error) => write!(f, "некорректное целое число: {error}"),
        }
    }
}

impl Error for MathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MathError::DivisionByZero => None,
            MathError::InvalidNumber(error) => Some(error),
        }
    }
}

impl From<ParseIntError> for MathError {
    fn from(error: ParseIntError) -> Self {
        Self::InvalidNumber(error)
    }
}

pub type MathResult<T> = Result<T, MathError>;

pub fn divide(a: i32, b: i32) -> MathResult<i32> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Парсит оба аргумента и делит их, распространяя ошибки через `?`.
pub fn parse_and_divide(a: &str, b: &str) -> MathResult<i32> {
    let a: i32 = a.parse()?;
    let b: i32 = b.parse()?;
    divide(a, b)
}

/// Пример универсального типа ошибки для разных источников.
pub fn boxed_parse(input: &str) -> Result<i32, Box<dyn Error>> {
    let value = input.parse()?;
    Ok(value)
}

/// Unit-тип можно указать явно, хотя обычно его опускают.
pub fn print_unit_example() {
    println!("Функция возвращает unit-тип ()");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_question_mark_returns_none_early() {
        assert_eq!(add(Some(2), Some(3)), Some(5));
        assert_eq!(add(None, Some(3)), None);
        assert_eq!(add(Some(2), None), None);
        assert_eq!(add(None, None), None);
    }

    #[test]
    fn partial_function_uses_option() {
        assert_eq!(reciprocal(4.0), Some(0.25));
        assert_eq!(reciprocal(0.0), None);
        assert_eq!(reciprocal(0.0).unwrap_or(1.0), 1.0);
    }

    #[test]
    fn optional_field_can_be_read_and_taken() {
        let mut user = User::new("Анна", Some("anny"));
        assert_eq!(user.display_name(), "anny");
        assert_eq!(user.take_nickname(), Some("anny".to_owned()));
        assert_eq!(user.nickname, None);
        assert_eq!(user.display_name(), "Анна");
    }

    #[test]
    fn result_reports_division_by_zero() {
        assert_eq!(divide(10, 2).ok(), Some(5));
        assert!(matches!(divide(10, 0), Err(MathError::DivisionByZero)));
        assert_eq!(divide(10, 0).unwrap_err().to_string(), "деление на ноль");
    }

    #[test]
    fn parse_error_is_converted_and_preserved_as_source() {
        let error = parse_and_divide("abc", "2").unwrap_err();
        assert!(matches!(error, MathError::InvalidNumber(_)));
        assert!(error.source().is_some());
        assert!(error.to_string().contains("некорректное целое число"));
    }

    #[test]
    fn parse_and_divide_propagates_errors() {
        assert_eq!(parse_and_divide("10", "4").ok(), Some(2));
        assert!(parse_and_divide("10", "0").is_err());
        assert!(parse_and_divide("10", "bad").is_err());
    }

    #[test]
    fn boxed_error_accepts_parse_error() {
        assert_eq!(boxed_parse("42").ok(), Some(42));
        assert!(boxed_parse("bad").is_err());
    }
}
