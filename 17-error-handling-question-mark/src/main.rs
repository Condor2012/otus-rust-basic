use std::num::ParseIntError;

// LIVE, слайд 15: паника используется при нарушении контракта функции.
fn parse_port_or_panic(input: &str) -> u16 {
    input
        .parse()
        .expect("порт должен быть корректным числом u16")
}

// LIVE, слайд 21: обработчики Option.
fn find_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn double_if_positive(value: Option<i32>) -> Option<i32> {
    value
        .and_then(|number| (number > 0).then_some(number))
        .map(|number| number * 2)
}

fn first_present(primary: Option<String>, fallback: Option<String>) -> Option<String> {
    primary.or_else(|| fallback)
}

fn option_handlers_demo() {
    let value = find_even(&[1, 3, 4, 6]);
    println!("is_some: {}, is_none: {}", value.is_some(), value.is_none());
    println!("map: {:?}", value.map(|number| number * 10));
    println!("and: {:?}", value.and(Some(100)));
    println!("and_then, положительное: {:?}", double_if_positive(Some(3)));
    println!("and_then, отрицательное: {:?}", double_if_positive(Some(-3)));
    println!(
        "or_else: {:?}",
        first_present(None, Some("резервное значение".to_owned()))
    );
    println!("unwrap_or: {}", None.unwrap_or(42));
    println!("unwrap_or_else: {}", None.unwrap_or_else(|| 6 * 7));
    println!("unwrap_or_default: {}", Option::<i32>::None.unwrap_or_default());
    println!("expect на Some: {}", value.expect("чётное значение найдено"));
}

// LIVE, слайд 26: обработчики Result.
fn parse_positive(input: &str) -> Result<i32, String> {
    input
        .parse::<i32>()
        .map_err(|error| format!("не удалось разобрать '{input}': {error}"))
        .and_then(|number| {
            if number > 0 {
                Ok(number)
            } else {
                Err(format!("число {number} не является положительным"))
            }
        })
}

fn result_handlers_demo() {
    let parsed = parse_positive("5");
    println!("is_ok: {}, is_err: {}", parsed.is_ok(), parsed.is_err());
    println!("map: {:?}", parsed.map(|number| number * 2));
    println!(
        "map_err: {:?}",
        parse_positive("bad").map_err(|error| error.to_uppercase())
    );
    println!("and: {:?}", parsed.and(Ok(100)));
    println!("and_then: {:?}", parsed.and_then(|number| parse_positive(&number.to_string())));
    println!("or: {:?}", parse_positive("bad").or(Ok(7)));
    println!(
        "or_else: {:?}",
        parse_positive("-1").or_else(|_| parse_positive("8"))
    );
    println!("unwrap_or: {:?}", parse_positive("bad").unwrap_or(42));
    println!(
        "unwrap_or_else: {:?}",
        parse_positive("bad").unwrap_or_else(|error| error.len() as i32)
    );
    println!(
        "unwrap_or_default: {:?}",
        parse_positive("bad").unwrap_or_default()
    );
}

// LIVE, слайд 29: ? возвращает Err сразу после неудачного разбора.
fn multiply_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a * b)
}

// Оператор ? аналогично выполняет ранний возврат None.
fn sum_first_and_last(values: &[i32]) -> Option<i32> {
    let first = values.first()?;
    let last = values.last()?;
    Some(first + last)
}

fn main() {
    println!("=== LIVE: паника ===");
    println!("Корректный порт: {}", parse_port_or_panic("8080"));
    println!("Некорректное значение вызвало бы панику, поэтому здесь оно не передаётся");

    println!("\n=== LIVE: Option ===");
    option_handlers_demo();

    println!("\n=== LIVE: Result ===");
    result_handlers_demo();

    println!("\n=== LIVE: оператор ? ===");
    println!("6 * 7 = {:?}", multiply_strings("6", "7"));
    println!("Ошибка разбора: {:?}", multiply_strings("6", "bad"));
    println!("Первый + последний: {:?}", sum_first_and_last(&[1, 2, 3]));
    println!("Пустой срез: {:?}", sum_first_and_last(&[]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port_is_returned() {
        assert_eq!(parse_port_or_panic("8080"), 8080);
    }

    #[test]
    #[should_panic(expected = "порт должен быть корректным числом u16")]
    fn invalid_port_panics() {
        parse_port_or_panic("bad");
    }

    #[test]
    fn option_handlers_transform_and_fallback() {
        assert_eq!(find_even(&[1, 3, 5]), None);
        assert_eq!(find_even(&[1, 4, 6]), Some(4));
        assert_eq!(double_if_positive(Some(3)), Some(6));
        assert_eq!(double_if_positive(Some(-3)), None);
        assert_eq!(first_present(None, Some("fallback".to_owned())), Some("fallback".to_owned()));
        assert_eq!(Option::<i32>::None.unwrap_or_default(), 0);
    }

    #[test]
    fn result_handlers_transform_values_and_errors() {
        assert_eq!(parse_positive("5"), Ok(5));
        assert!(parse_positive("0").is_err());
        assert!(parse_positive("bad").is_err());
        assert_eq!(parse_positive("bad").or(Ok(7)), Ok(7));
        assert_eq!(parse_positive("bad").unwrap_or_default(), 0);
    }

    #[test]
    fn question_mark_propagates_result_errors() {
        assert_eq!(multiply_strings("6", "7"), Ok(42));
        assert!(multiply_strings("bad", "7").is_err());
        assert!(multiply_strings("6", "bad").is_err());
    }

    #[test]
    fn question_mark_propagates_none() {
        assert_eq!(sum_first_and_last(&[1, 2, 3]), Some(4));
        assert_eq!(sum_first_and_last(&[5]), Some(10));
        assert_eq!(sum_first_and_last(&[]), None);
    }
}
