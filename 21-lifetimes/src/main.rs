use lesson_21_lifetimes::{LtStruct, first_word, longest, static_number, static_text};

fn main() {
    let first = String::from("short");
    let second = String::from("a much longer string");
    let result = longest(&first, &second);
    println!("Самая длинная строка: {result}");

    let text = String::from("Rust проверяет ссылки на этапе компиляции");
    let word: &str = first_word(&text);
    println!("Первое слово без явной аннотации: {word}");

    let a = 10_u32;
    let b = 20_u32;
    let value = LtStruct {
        val: 1,
        p_val1: &a,
        p_val2: &b,
    };
    println!("Метод get_ptr: {}", value.get_ptr());
    println!("Сумма полей и ссылок: {}", value.sum());

    let number: &'static i32 = static_number();
    let static_str: &'static str = static_text();
    println!("Статическое число: {number}");
    println!("Статическая строка: {static_str}");

    let mut mutable_value = 42_i32;
    let mutable_ref: &mut i32 = &mut mutable_value;
    *mutable_ref += 1;
    println!("Изменение через &mut i32: {mutable_value}");
}
