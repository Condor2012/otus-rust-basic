use collections_slices_strings::{choose_elem, get_nth};

fn main() {
    let mut pair = (10, 20);
    let selected = choose_elem(&mut pair, true);
    *selected += 5;
    println!("Кортеж после изменения второго элемента: {pair:?}");

    let mut values = [1, 2, 3, 4, 5];
    let third = get_nth(&mut values[..3], 2);
    *third *= 10;
    println!("Массив после изменения через срез: {values:?}");
}
