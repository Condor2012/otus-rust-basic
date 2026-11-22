use advanced_syntax::{
    associated_types::{Graph, SimpleGraph}, attributes::{User, square}, closures,
    conditional_compilation, deref::{MyBox, text_len}, guards::{Point, locate},
    let_patterns, lifetimes, patterns::{Message, describe_message}, traits::Described,
};

fn main() {
    println!("{}", describe_message(&Message::Move { x: 2, y: 3 }));
    println!("{}", locate(Point { x: 2, y: -2 }));
    println!("if let + условие: {:?}", let_patterns::greater_than_ten(Some(11)));
    println!("Замыкание Fn: {}", closures::apply(|x| x * 2));
    println!("Атрибуты: {:?}, {}", User { name: "Анна".into() }, square(4));
    println!("Время жизни: {}", lifetimes::longest("длинная", "короткая"));

    let graph = SimpleGraph { connections: vec![(1, 2), (1, 3)] };
    println!("Ассоциированные типы: {:?}", graph.edges(&1));

    let text = MyBox::new(String::from("Привет"));
    println!("Deref: длина {}", text_len(&text));
    println!("Условная компиляция: {}", conditional_compilation::system_info());
    println!("Blanket impl: {}", 42.describe());
    println!("Макрос: {:?}", advanced_syntax::make_vec![1, 2, 3]);
}
