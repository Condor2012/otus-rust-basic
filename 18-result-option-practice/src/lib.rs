use std::io;

pub fn last_char(value: &str) -> Option<char> {
    value.chars().last()
}

pub fn safe_divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

pub fn find_element(values: &[i32], target: i32) -> Option<usize> {
    values.iter().position(|value| *value == target)
}

pub fn safe_sqrt(value: f64) -> Option<f64> {
    if value < 0.0 {
        None
    } else {
        Some(value.sqrt())
    }
}

pub fn sum_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.and_then(|x| b.map(|y| x + y))
}

pub fn checked_divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err("Division by zero".to_owned())
    } else {
        Ok(numerator / denominator)
    }
}

pub fn parse_number(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map_err(|_| "Invalid number".to_owned())
}

pub fn read_file_contents(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}

pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("Password too short".to_owned())
    } else if !password.chars().any(|ch| ch.is_ascii_digit()) {
        Err("Password must contain a digit".to_owned())
    } else {
        Ok(())
    }
}

pub fn sum_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    let a = a?;
    let b = b?;
    Ok(a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_last_char() {
        assert_eq!(last_char("Hello"), Some('o'));
        assert_eq!(last_char("Привет"), Some('т'));
        assert_eq!(last_char(""), None);
    }

    #[test]
    fn option_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(safe_divide(10.0, 0.0), None);
        assert_eq!(safe_divide(10.0, -0.0), None);
    }

    #[test]
    fn option_find_element() {
        let values = vec![1, 2, 3, 4, 3];
        assert_eq!(find_element(&values, 3), Some(2));
        assert_eq!(find_element(&values, 5), None);
        assert_eq!(find_element(&[], 1), None);
    }

    #[test]
    fn option_safe_sqrt() {
        assert_eq!(safe_sqrt(4.0), Some(2.0));
        assert_eq!(safe_sqrt(9.0), Some(3.0));
        assert_eq!(safe_sqrt(0.0), Some(0.0));
        assert_eq!(safe_sqrt(-1.0), None);
    }

    #[test]
    fn option_sum() {
        assert_eq!(sum_options(Some(5), Some(3)), Some(8));
        assert_eq!(sum_options(Some(5), None), None);
        assert_eq!(sum_options(None, Some(3)), None);
        assert_eq!(sum_options(None, None), None);
    }

    #[test]
    fn result_checked_divide() {
        assert_eq!(checked_divide(10, 2), Ok(5));
        assert_eq!(checked_divide(10, 0), Err("Division by zero".to_owned()));
    }

    #[test]
    fn result_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-7"), Ok(-7));
        assert_eq!(parse_number("abc"), Err("Invalid number".to_owned()));
    }

    #[test]
    fn result_read_file() {
        let path = std::env::temp_dir().join(format!(
            "lesson-18-result-option-{}.txt",
            std::process::id()
        ));
        std::fs::write(&path, "Hello, world!").expect("временный файл должен быть создан");

        let result = read_file_contents(path.to_str().expect("путь должен быть UTF-8"));
        std::fs::remove_file(&path).expect("временный файл должен быть удалён");

        assert_eq!(result.ok(), Some("Hello, world!".to_owned()));
        assert!(read_file_contents("definitely-nonexistent-lesson-18.txt").is_err());
    }

    #[test]
    fn result_validate_password() {
        assert_eq!(validate_password("password1"), Ok(()));
        assert_eq!(
            validate_password("short"),
            Err("Password too short".to_owned())
        );
        assert_eq!(
            validate_password("noligits"),
            Err("Password must contain a digit".to_owned())
        );
    }

    #[test]
    fn result_sum_returns_first_error() {
        assert_eq!(sum_results(Ok(2), Ok(3)), Ok(5));
        assert_eq!(
            sum_results(Ok(2), Err("error".to_owned())),
            Err("error".to_owned())
        );
        assert_eq!(
            sum_results(Err("fail".to_owned()), Ok(3)),
            Err("fail".to_owned())
        );
        assert_eq!(
            sum_results(Err("first".to_owned()), Err("second".to_owned())),
            Err("first".to_owned())
        );
    }
}
