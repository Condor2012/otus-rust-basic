use lesson_15_type_system_result_option::{
    User, add, boxed_parse, parse_and_divide, print_unit_example, reciprocal,
};

fn main() {
    println!("Some(2) + Some(3) = {:?}", add(Some(2), Some(3)));
    println!("None + Some(3) = {:?}", add(None, Some(3)));

    for value in [4.0, 0.0] {
        match reciprocal(value) {
            Some(result) => println!("1 / {value} = {result}"),
            None => println!("Для {value} обратного значения нет"),
        }
    }

    let mut user = User::new("Анна", Some("anny"));
    println!("Отображаемое имя: {}", user.display_name());
    let nickname = user.take_nickname();
    println!("Извлечённый псевдоним: {nickname:?}");
    println!("Имя после take: {}", user.display_name());

    for (a, b) in [("10", "4"), ("10", "0"), ("abc", "2")] {
        match parse_and_divide(a, b) {
            Ok(result) => println!("{a} / {b} = {result}"),
            Err(error) => println!("Не удалось вычислить {a} / {b}: {error}"),
        }
    }

    println!("Box<dyn Error>, успешный разбор: {:?}", boxed_parse("42"));
    print_unit_example();
}
