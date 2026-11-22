use lesson_16_closures_introduction::{
    DynProcessor, GenericProcessor, accumulate_with, apply_function, call_twice, consume_once,
    get_adder, square, use_hrtb,
};

fn main() {
    println!("Указатель на функцию: {}", apply_function(square, 5));

    let implicit = |a, b| a + b;
    let explicit = |a: u32, b: u32| -> u32 { a + b };
    println!("Неявная сигнатура: {}", implicit(2, 3));
    println!("Явная сигнатура: {}", explicit(2, 3));

    let offset = 10_u32;
    let add_offset = |value| value + offset;
    println!("Fn, два вызова: {:?}", call_twice(add_offset, 5));

    let mut sum = 0_u32;
    accumulate_with(|value| sum += value, &[1, 2, 3]);
    println!("FnMut изменил внешнюю сумму: {sum}");

    let mut adder = get_adder(10);
    println!("Возвращённое замыкание: {}, {}, {}", adder(1), adder(2), adder(3));

    let text = String::from("FnOnce поглощает эту строку");
    let consuming_closure = move || text;
    println!("{}", consume_once(consuming_closure));

    let generic = GenericProcessor {
        function: |value| value * 2,
    };
    let dynamic = DynProcessor {
        function: Box::new(|value| value + 100),
    };
    println!("Обобщённое хранение: {}", generic.process(10));
    println!("Box<dyn Fn>: {}", dynamic.process(10));

    let mut value = 10_u32;
    let selected = use_hrtb(|first, _second| first, &mut value);
    *selected += 5;
    println!("HRTB, изменённое значение: {value}");
}
