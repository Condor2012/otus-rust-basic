use lesson39_proc_macros::Count;
use macros_lesson::{add, four, hashmap, make_vec, using_a};

#[allow(dead_code)]
#[derive(Count)]
enum Example {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
    println!("2 * four!() = {}", 2 * four!());
    println!("add!(5, 1 + 3) = {}", add!(5, 1 + 3));
    println!("make_vec!: {:?}", make_vec![1, 2, 3]);
    println!("hashmap!: {:?}", hashmap! { 1 => "one", 2 => "two" });
    println!("using_a!: {}", using_a!(a, a / 10));
    println!("В enum Example {} варианта", Example::count());
}
