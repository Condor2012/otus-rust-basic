use std::cell::RefCell;

fn main() {
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
