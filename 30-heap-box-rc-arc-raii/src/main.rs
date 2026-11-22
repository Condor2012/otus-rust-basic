use lesson_30_heap_box_rc_arc_raii::arc_demo::{arc_between_threads, arc_weak_upgrade_states};
use lesson_30_heap_box_rc_arc_raii::boxed::{List, boxed_slice, boxed_trait, boxed_value};
use lesson_30_heap_box_rc_arc_raii::raii::drops_after_scope;
use lesson_30_heap_box_rc_arc_raii::rc_demo::{rc_counts, weak_upgrade_states};

fn main() {
    println!("RAII: ресурсов освобождено после области видимости: {}", drops_after_scope());

    println!("Box<i32>: {}", *boxed_value(42));
    println!("Box<[i32]>: {:?}", boxed_slice(&[1, 2, 3]));
    println!("Box<dyn Number>: {}", boxed_trait(7).value());

    let list = List::from_slice(&[1, 2, 3]);
    println!("Рекурсивный список: {list:?}, длина: {}, сумма: {}", list.len(), list.sum());

    println!("Rc: исходно, внутри области, после: {:?}", rc_counts());
    println!("Rc Weak: upgrade при владельце и после drop: {:?}", weak_upgrade_states());
    println!("Arc между потоками: {:?}", arc_between_threads());
    println!("Arc Weak: upgrade при владельце и после drop: {:?}", arc_weak_upgrade_states());
}
