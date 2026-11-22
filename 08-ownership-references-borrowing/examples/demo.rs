use std::mem::size_of;

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn get_second_word(s: &str) -> Option<&str> {
    s.split_whitespace().nth(1)
}

fn sum_bytes(bytes: &[u8]) -> u32 {
    bytes.iter().map(|byte| u32::from(*byte)).sum()
}

fn owned_string() -> String {
    String::from("Строка возвращается вместе с владением")
}

fn main() {
    println!("=== Указатель как адрес ===");
    let number = 42_i32;
    let pointer = &number as *const i32;
    println!("Значение: {number}, адрес: {pointer:?}");
    println!("Размер указателя: {} байт", size_of::<*const i32>());

    println!("\n=== Разделяемые ссылки ===");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Длина строки '{s1}' равна {len}; s1 всё ещё доступна");

    let r1 = &s1;
    let r2 = &s1;
    println!("Две разделяемые ссылки: {r1} и {r2}");

    println!("\n=== Эксклюзивная ссылка ===");
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Изменённая строка: {s2}");

    let shared1 = &s2;
    let shared2 = &s2;
    println!("Чтение через ссылки: {shared1}, {shared2}");
    // После последнего использования shared1 и shared2 можно создать &mut String.
    let exclusive = &mut s2;
    exclusive.push('!');
    println!("После эксклюзивного заимствования: {exclusive}");

    println!("\n=== Без висячих ссылок ===");
    let owned = owned_string();
    println!("{owned}");

    println!("\n=== Толстые ссылки и срезы ===");
    let text = String::from("Rust делает заимствование безопасным");
    let second = get_second_word(&text).expect("в строке есть второе слово");
    println!("Второе слово: {second}");
    println!("Размер &str: {} байт", size_of::<&str>());
    println!("Размер &[u8]: {} байт", size_of::<&[u8]>());

    let bytes = vec![1_u8, 2, 3, 4];
    println!("Сумма байтов Vec: {}", sum_bytes(&bytes));
    println!("Сумма части Vec: {}", sum_bytes(&bytes[1..3]));

    println!("\n=== Умные указатели ===");
    let boxed = Box::new(42_u32);
    println!("Box владеет значением в куче: {}", *boxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_reference_preserves_ownership() {
        let value = String::from("hello");
        assert_eq!(calculate_length(&value), 5);
        assert_eq!(value, "hello");
    }

    #[test]
    fn exclusive_reference_changes_value() {
        let mut value = String::from("hello");
        change(&mut value);
        assert_eq!(value, "hello, world");
    }

    #[test]
    fn second_word_is_a_str_slice() {
        let value = String::from("hello borrowed world");
        assert_eq!(get_second_word(&value), Some("borrowed"));
        assert_eq!(get_second_word("one"), None);
    }

    #[test]
    fn byte_slices_have_the_same_interface() {
        let values = vec![1_u8, 2, 3, 4];
        assert_eq!(sum_bytes(&values), 10);
        assert_eq!(sum_bytes(&values[1..3]), 5);
        assert_eq!(sum_bytes(&[]), 0);
    }
}
