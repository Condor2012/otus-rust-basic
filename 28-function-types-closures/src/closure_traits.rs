/// Fn: лямбду можно вызывать многократно, не изменяя захваченное окружение.
pub fn calculate_with_fn<F>(a: i32, b: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(a, b)
}

/// FnMut: лямбда может изменять захваченные переменные.
pub fn accumulate_with_fn_mut<F>(values: &[i32], mut operation: F) -> i32
where
    F: FnMut(i32),
{
    for &value in values {
        operation(value);
    }
    values.iter().sum()
}

/// FnOnce: лямбда может забрать владение захваченным значением.
pub fn consume_with_fn_once<F>(operation: F) -> String
where
    F: FnOnce() -> String,
{
    operation()
}

pub fn examples() -> (i32, i32, String) {
    let factor = 3;
    let fn_result = calculate_with_fn(2, 4, |a, b| (a + b) * factor);

    let mut calls = 0;
    let fn_mut_result = accumulate_with_fn_mut(&[1, 2, 3], |_| calls += 1);
    assert_eq!(calls, 3);

    let greeting = String::from("Владение передано лямбде");
    let fn_once_result = consume_with_fn_once(move || greeting);

    (fn_result, fn_mut_result, fn_once_result)
}

#[cfg(test)]
mod tests {
    use super::{accumulate_with_fn_mut, calculate_with_fn, consume_with_fn_once, examples};

    #[test]
    fn demonstrates_all_three_closure_traits() {
        assert_eq!(examples(), (18, 6, String::from("Владение передано лямбде")));
    }

    #[test]
    fn fn_can_read_captured_environment() {
        let offset = 10;
        assert_eq!(calculate_with_fn(1, 2, |a, b| a + b + offset), 13);
    }

    #[test]
    fn fn_mut_can_change_captured_environment() {
        let mut total = 0;
        accumulate_with_fn_mut(&[2, 4, 6], |value| total += value);
        assert_eq!(total, 12);
    }

    #[test]
    fn fn_once_can_move_captured_value() {
        let text = String::from("Rust");
        assert_eq!(consume_with_fn_once(move || text), "Rust");
    }
}
