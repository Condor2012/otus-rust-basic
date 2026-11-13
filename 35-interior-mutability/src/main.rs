use core::borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::RefCell;
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



    let items = RefCell::new(vec!["яблоко", "банан"]);
    println!("Начальное список: {:?}", items.borrow());

    {
        let mut items_ref = items.borrow_mut();
        items_ref.push("апельсин");
        println!("Добавили апельсин: {:?}", items_ref);
    }

    println!("После добавления: {:?}", items.borrow());

    {
        let mut items_ref = items.borrow_mut();
        items_ref.pop();
        println!("Удалили последний элемент: {:?}", items_ref);
    }

    println!("Финальный список: {:?}", items.borrow());

    let reader1 = items.borrow();
    let reader2 = items.borrow();
    println!("Читатель 1: {:?}", reader1);
    println!("Читатель 2: {:?}", reader2);

}
