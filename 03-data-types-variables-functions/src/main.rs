#![allow(clippy::vec_init_then_push)]

// Занятие 3. Типы данных. Переменные и функции

// Константы: аннотация типа обязательна, имя в SCREAMING_SNAKE_CASE.
// Значение вычисляется на этапе компиляции и подставляется в места использования.
const NUM: u32 = 42;
const ARR: [u32; 3] = [0, NUM, NUM * 2];
const STRING: &str = "string";

fn main() {
    // ---- Переменные ----
    // let с аннотацией типа
    let y: u32 = 0;
    println!("y = {y}");

    // Аннотация опциональна: тип выводится по значению
    let x = 0; // i32 по умолчанию
    println!("x = {x}");

    // Изменяемая переменная — mut
    let mut year = 0;
    println!("year = {year}");
    year = 2024;
    println!("year = {year}");

    // Затенение (shadowing): новое let с тем же именем
    let y = 2023;
    let y = y + 1;
    {
        let y = y - 24;
        println!("внутренний y = {y}"); // 2000
    }
    println!("внешний y = {y}"); // 2024

    // Затенение можно использовать, чтобы «заморозить» переменную
    #[allow(clippy::vec_init_then_push)]
    let mut v = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    let v = v; // теперь v неизменяем
    println!("v = {v:?}");

    // ---- Целые числа и литералы ----
    let a = 2024; // i32 по умолчанию
    let b: u16 = 2024;
    let c = 2024u16; // суффикс типа в литерале
    let d = 2_024; // подчёркивания для читаемости
    let hex = 0xffff; // шестнадцатеричный
    let oct = 0o7777; // восьмеричный
    let bin = 0b1111_1111; // двоичный
    let byte = b'A'; // байтовый литерал: u8
    println!("{a} {b} {c} {d} {hex} {oct} {bin} {byte}");

    // ---- Числовые операции ----
    println!("2 + 2 = {}", 2 + 2);
    println!("10 / 3 = {}", 10 / 3); // целочисленное деление
    println!("10 % 3 = {}", 10 % 3);
    println!("4 << 2 = {}", 4 << 2); // сдвиг влево
    println!("5 >> 1 = {}", 5 >> 1); // сдвиг вправо
    println!("!15_u8 = {}", !15_u8); // побитовый NOT
    println!("5 & 3 = {}", 5 & 3); // AND
    println!("5 | 3 = {}", 5 | 3); // OR
    println!("5 ^ 3 = {}", 5 ^ 3); // XOR

    // Методы целых чисел
    println!("abs: {}", 2024_i32.abs());
    println!("pow: {}", 2024_u64.pow(3));
    println!("rotate_left: {}", 2024_u32.rotate_left(3));
    println!("count_ones: {}", 2024_u32.count_ones());
    println!("to_be_bytes: {:?}", 2024_u32.to_be_bytes());
    println!("from_be_bytes: {}", u32::from_be_bytes([0, 0, 7, 232]));

    // ---- Переполнение ----
    // В отладочной сборке value + 2024 при value = u32::MAX вызовет панику.
    // Поэтому используем явные стратегии:
    assert_eq!(253_u8.checked_add(1), Some(254));
    assert_eq!(253_u8.checked_add(3), None);
    assert_eq!(253_u8.wrapping_add(3), 0);
    assert_eq!(253_u8.overflowing_add(3), (0, true));
    assert_eq!(253_u8.saturating_add(3), u8::MAX);
    println!("checked: {:?}", 253_u8.checked_add(3));
    println!("wrapping: {}", 253_u8.wrapping_add(3));
    println!("overflowing: {:?}", 253_u8.overflowing_add(3));
    println!("saturating: {}", 253_u8.saturating_add(3));

    // ---- Числа с плавающей точкой ----
    let f = 2.1; // f64 по умолчанию
    let g: f32 = 2.1;
    let h = 2_e3; // 2000.0
    println!("{f} {g} {h}");
    println!("10.0 / 3.0 = {}", 10.0 / 3.0);
    println!("ceil: {}", 1.23_f32.ceil());
    println!("floor: {}", 1.23_f32.floor());
    println!("round: {}", 1.23_f32.round());
    println!("sqrt: {}", 1.23_f32.sqrt());
    println!("powi: {}", 1.23_f32.powi(2));

    // ---- bool ----
    let flag1 = true;
    let flag2: bool = false;
    println!("!flag1 = {}", !flag1);
    println!("flag1 && flag2 = {}", flag1 && flag2);
    println!("flag1 || flag2 = {}", flag1 || flag2);

    // ---- char ----
    // 32 бита, содержит символ Unicode
    let c = 'a';
    let heart = '❤';
    let cyrillic = 'ж';
    let emoji = '🚀';
    println!("{c} {heart} {cyrillic} {emoji}");

    // ---- Приведение типов ----
    println!("2024_u16 as u8 = {}", 2024_u16 as u8); // 232
    println!("2.2 as u32 = {}", 2.2_f64 as u32); // 2
    println!("true as u8 = {}", true as u8); // 1
    println!("(-1_i8) as u8 = {}", (-1_i8) as u8); // 255
    println!("'a' as u32 = {}", 'a' as u32); // 97

    // ---- Единичный тип ----
    let u: () = (); // нулевой размер; результат функции без возврата
    println!("unit: {u:?}");

    // ---- Массивы ----
    // Фиксированный размер, может размещаться на стеке
    let arr = [1, 2, 3];
    let fives = [5; 3]; // [5, 5, 5]
    println!("arr = {arr:?}, fives = {fives:?}");
    println!("arr[0] = {}", arr[0]);
    // .get() — безопасный доступ: None вместо паники при выходе за границы
    match arr.get(10) {
        Some(item) => println!("элемент: {item}"),
        None => println!("индекс 10 вне массива"),
    }

    // ---- Строки ----
    let str_slice: &str = "abc"; // строковый срез
    let owned: String = "abc".to_owned(); // String владеет содержимым
    println!("{str_slice} / {owned}");
    println!("len = {}", "abcdef".len());
    println!("chars = {}", "abcdef".chars().count());
    println!("срез: {}", &"abcdef"[2..]);
    println!("starts_with: {}", "abcdef".starts_with("abc"));
    println!("find: {:?}", "abcdef".find("cd"));
    println!("replace: {}", "abcdef".replace("abc", "ABC"));

    // ---- Функции ----
    println!("foo(5, 3) = {}", foo(5, 3));
    println!("foo_with_locals(5, 3) = {}", foo_with_locals(5, 3));
    println!("add(2, 2) = {}", add(2, 2));
    println!("early_return(200) = {}", early_return(200));
    println!("early_return(50) = {}", early_return(50));
    println!("str_to_int(\"123\") = {}", str_to_int("123"));

    // Функция как аргумент
    println!("any(&[1, 3, 5], is_even) = {}", any(&[1, 3, 5], is_even));
    println!("any(&[1, 3, 4], is_even) = {}", any(&[1, 3, 4], is_even));

    // ---- Константы ----
    println!("NUM = {NUM}, NUM + 1 = {}", NUM + 1);
    println!("ARR = {ARR:?}, ARR[2] = {}", ARR[2]);
    println!("STRING = {STRING}");
    // const fn: можно вызывать и на этапе компиляции, и в рантайме
    println!("double(1) на этапе компиляции = {DOUBLE}");
    println!("double(2) в рантайме = {}", double(2));
}

/// Тело функции: последнее выражение без `;` — возвращаемое значение.
fn foo(a: i32, b: i32) -> i32 {
    (a + b) * (a - b)
}

/// То же самое с локальными переменными.
fn foo_with_locals(a: i32, b: i32) -> i32 {
    let x = a + b;
    let y = a - b;
    x * y
}

/// Явный возврат через return.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// return как оператор раннего возврата.
fn early_return(x: u32) -> u32 {
    if x > 100 {
        return 0;
    }
    x + 1
}

/// Тип результата parse выводится из сигнатуры функции.
fn str_to_int(s: &str) -> u32 {
    s.parse().unwrap()
}

fn is_even(i: u32) -> bool {
    i.is_multiple_of(2)
}

/// Функция принимает другую функцию как аргумент (тип fn).
fn any(arr: &[u32], f: fn(u32) -> bool) -> bool {
    for item in arr {
        if f(*item) {
            return true;
        }
    }
    false
}

/// const fn вычислима на этапе компиляции.
const fn double(x: u32) -> u32 {
    x + x
}

const DOUBLE: u32 = double(1);
