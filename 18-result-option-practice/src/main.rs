use lesson_18_result_option_practice::{
    checked_divide, find_element, last_char, parse_number, read_file_contents, safe_divide,
    safe_sqrt, sum_options, sum_results, validate_password,
};

fn main() {
    println!("=== Option ===");
    println!("Последний символ 'Hello': {:?}", last_char("Hello"));
    println!("10 / 2: {:?}", safe_divide(10.0, 2.0));
    println!("10 / 0: {:?}", safe_divide(10.0, 0.0));
    println!("Индекс числа 3: {:?}", find_element(&[1, 2, 3, 4], 3));
    println!("Корень из 9: {:?}", safe_sqrt(9.0));
    println!("Корень из -1: {:?}", safe_sqrt(-1.0));
    println!("Some(5) + Some(3): {:?}", sum_options(Some(5), Some(3)));

    println!("\n=== Result ===");
    println!("10 / 2: {:?}", checked_divide(10, 2));
    println!("10 / 0: {:?}", checked_divide(10, 0));
    println!("Разбор '42': {:?}", parse_number("42"));
    println!("Разбор 'abc': {:?}", parse_number("abc"));
    println!(
        "Чтение Cargo.toml: {}",
        read_file_contents("Cargo.toml").map(|contents| contents.lines().count())
    );
    println!("Пароль password1: {:?}", validate_password("password1"));
    println!("Пароль short: {:?}", validate_password("short"));
    println!("Ok(2) + Ok(3): {:?}", sum_results(Ok(2), Ok(3)));
    println!(
        "Ok(2) + Err: {:?}",
        sum_results(Ok(2), Err("error".to_owned()))
    );
}
