use lesson_24_generics_lifetimes_practice::difference::find_difference;
use lesson_24_generics_lifetimes_practice::matcher::Matcher;
use lesson_24_generics_lifetimes_practice::tuple_sum::tuple_sum2;
use lesson_24_generics_lifetimes_practice::vector::Vector;

fn main() {
    let first = "I hate the surf and the sand.";
    let second = "I love the snow and the sand.";
    println!("Разница предложений: {:?}", find_difference(first, second));

    println!("tuple_sum2((5, 3)) = {}", tuple_sum2((5, 3)));
    println!(
        "tuple_sum2((5.0, 3.0, \"hello\")) = {}",
        tuple_sum2((5.0, 3.0, "hello"))
    );
    println!(
        "tuple_sum2((10, 20, 30, 40, 50)) = {}",
        tuple_sum2((10, 20, 30, 40, 50))
    );

    let vector = Vector::new3(2.0, 3.0, 6.0);
    println!("Вектор: {vector:?}, длина: {}", vector.len());
    println!("Сложение: {:?}", vector + Vector::new3(1.0, 1.0, 1.0));
    println!("Умножение на 2: {:?}", vector * 2.0);

    let pattern = "abc(d|e|f).".to_string();
    let candidate = "abcde".to_string();
    let mut matcher = Matcher::new(&pattern).unwrap();
    println!("Совпадения: {:?}", matcher.match_string(&candidate));
    println!(
        "Шаблон {:?}, максимум совпавших токенов: {}",
        matcher.text(),
        matcher.most_tokens_matched()
    );
}
