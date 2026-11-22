pub static X: i32 = 777;

/// Возвращает более длинную строку.
///
/// Результат живёт не дольше, чем более короткое из двух входных заимствований.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// Благодаря правилам элизии сигнатура эквивалентна `first_word<'a>(&'a str) -> &'a str`.
pub fn first_word(value: &str) -> &str {
    value.split_whitespace().next().unwrap_or("")
}

/// Структура хранит ссылки с двумя независимыми временами жизни.
pub struct LtStruct<'a, 'b> {
    pub val: u32,
    pub p_val1: &'a u32,
    pub p_val2: &'b u32,
}

impl<'a, 'b> LtStruct<'a, 'b> {
    /// Возвращается исходная ссылка `'a`, поэтому её время жизни не ограничено `&self`.
    pub fn get_ptr(&self) -> &'a u32 {
        self.p_val1
    }

    pub fn sum(&self) -> u32 {
        self.val + *self.p_val1 + *self.p_val2
    }
}

pub fn static_number() -> &'static i32 {
    &X
}

pub fn static_text() -> &'static str {
    "I have a static lifetime."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_returns_one_of_inputs() {
        let first = String::from("short");
        let second = String::from("considerably longer");
        assert_eq!(longest(&first, &second), second);
        assert_eq!(longest(&second, &first), second);
    }

    #[test]
    fn longest_result_is_limited_to_input_lifetimes() {
        let first = String::from("long string");
        {
            let second = String::from("tiny");
            let result = longest(&first, &second);
            assert_eq!(result, "long string");
        }
        // Результат нельзя использовать здесь: сигнатура longest допускает ссылку на second,
        // даже если при конкретном запуске была выбрана ссылка на first.
        assert_eq!(first, "long string");
    }

    #[test]
    fn lifetime_elision_works_for_first_word() {
        let text = String::from("Rust lifetime rules");
        assert_eq!(first_word(&text), "Rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn struct_supports_two_lifetimes() {
        let first = 10;
        let second = 20;
        let value = LtStruct {
            val: 1,
            p_val1: &first,
            p_val2: &second,
        };

        assert_eq!(value.get_ptr(), &10);
        assert_eq!(value.sum(), 31);
    }

    #[test]
    fn static_references_live_for_entire_program() {
        assert_eq!(*static_number(), 777);
        assert_eq!(static_text(), "I have a static lifetime.");
    }
}
