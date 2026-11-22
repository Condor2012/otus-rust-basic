use std::collections::HashMap;

use lesson_19_generics_static_polymorphism::{
    IsNearlyZero, Klone, Pair, Point, add, get_value, largest, print_display, safe_div,
};

fn main() {
    let integers = Pair::new(1, 2);
    let strings = Pair::new("left", "right");
    println!("Pair<i32>: {integers:?}");
    println!("Pair<&str>: {strings:?}");

    println!("add(i32): {}", add(2_i32, 3));
    println!(
        "add(String): {}",
        add("Hello, ".to_owned(), "world".to_owned())
    );
    println!("largest: {:?}", largest(&[3, 1, 4, 2]));
    print_display(42);
    print_display("обобщённый вывод");

    println!("0_i32 почти ноль: {}", 0_i32.is_nearly_zero());
    println!("0.0000001_f32 почти ноль: {}", 0.0000001_f32.is_nearly_zero());
    println!("safe_div(10, 2): {:?}", safe_div(10_i32, 2));
    println!("safe_div(10, 0): {:?}", safe_div(10_i32, 0));

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    println!("Point::default(): {:?}", Point::default());
    println!("Сложение точек: {:?}", a + b);
    println!("Klone для i32: {}", 42_i32.klone());

    let mut map = HashMap::new();
    map.insert("answer".to_owned(), 42);
    println!("HashMap<String, i32>, поиск по &str: {:?}", get_value(&map, "answer"));
}
