use std::cell::Cell;

fn main() {
    let number = Cell::new(5);
    println!("Начальное значение: {}", number.get());

    number.set(10);
    println!("После Set(10): {}", number.get());

    let number_ref = &number;
    number_ref.set(15);
    println!("Через ссылку: {}", number.get());

    let old_value = number.replace(20);
    println!("Replace(20): старое =  {}, Новое = {}", old_value, number.get());

    let taken_value = number.take();
    println!("Take(): забрали =  {}, Осталось = {}", taken_value, number.get());
}
