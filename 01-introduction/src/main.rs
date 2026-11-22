fn main() {
    // println! — макрос вывода в консоль
    println!("Hello, world!");
    println!("Привет, Rust!");

    // Переменные по умолчанию неизменяемы (immutable)
    let name = "Otus";
    println!("Курс: {name}");

    // Для изменяемой переменной нужен mut
    let mut counter = 0;
    println!("counter = {counter}");
    counter += 1;
    println!("counter = {counter}");

    // Явное указание типа
    let count: i32 = 42;
    let price: f64 = 19.99;
    let is_active: bool = true;
    let initial: char = 'R';

    println!("count = {count}, price = {price}");
    println!("is_active = {is_active}, initial = {initial}");

    // Форматированный вывод
    println!("PI с точностью до 2 знаков: {:.2}", 3.14159265);
    println!("Двоичное представление 42: {:b}", 42);
}
