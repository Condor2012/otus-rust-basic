use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

fn move_player(direction: Direction) {
    println!("Двигаемся в направлении: {direction:?}");
}

fn get_status_message(status: OrderStatus) -> &'static str {
    use OrderStatus::*;

    match status {
        Pending => "В обработке",
        Shipped => "Отправлен",
        Delivered => "Доставлен",
        Cancelled => "Отменён",
    }
}

#[derive(Clone, Copy)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn get_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Стоп!",
        TrafficLight::Yellow => "Приготовиться",
        TrafficLight::Green => "Можно ехать",
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(message: Message) -> String {
    match message {
        Message::Quit => "Выходим из программы".to_owned(),
        Message::Move { x, y } => format!("Перемещаемся в точку ({x}, {y})"),
        Message::Write(text) => format!("Сообщение: {text}"),
        Message::ChangeColor(r, g, b) => format!("Меняем цвет на RGB({r}, {g}, {b})"),
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { a: f64, b: f64, c: f64 },
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle { a, b, c } => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

#[derive(Clone, Copy)]
enum Weather {
    Sunny,
    Rainy,
    Snowy,
    Cloudy,
}

fn should_take_umbrella(weather: Weather) -> bool {
    match weather {
        Weather::Rainy | Weather::Snowy => true,
        Weather::Sunny | Weather::Cloudy => false,
    }
}

fn analyze_number(x: i32) -> &'static str {
    match x {
        0 => "Ноль",
        1 | 2 | 3 => "Маленькое число",
        4..=10 => "Среднее число",
        _ => "Другое число",
    }
}

fn analyze_point(point: (i32, i32)) -> String {
    match point {
        (0, 0) => "Начало координат".to_owned(),
        (0, y) => format!("На оси Y: {y}"),
        (x, 0) => format!("На оси X: {x}"),
        (x, y) if x == y => format!("На диагонали: ({x}, {y})"),
        (x, y) => format!("Обычная точка: ({x}, {y})"),
    }
}

struct Person {
    name: String,
    age: u32,
}

fn describe_person(person: Person) -> String {
    match person {
        Person { name, age: 0 } => format!("{name} — новорождённый"),
        Person { name, age } if age < 18 => {
            format!("{name} — несовершеннолетний, {age} лет")
        }
        Person { name, age } => format!("{name} — взрослый, {age} лет"),
    }
}

enum ContactMethod {
    Email(String),
    Phone(String),
    Address { street: String, city: String },
}

struct Customer {
    name: String,
    contact: ContactMethod,
    vip: bool,
}

fn contact_customer(customer: Customer) -> String {
    match customer {
        Customer {
            name,
            contact: ContactMethod::Email(email),
            vip: true,
        } => format!("Отправляем VIP-письмо на {email} для {name}"),
        Customer {
            name,
            contact: ContactMethod::Email(email),
            vip: false,
        } => format!("Отправляем письмо на {email} для {name}"),
        Customer {
            name,
            contact: ContactMethod::Phone(phone),
            ..
        } => format!("Звоним {name} по номеру {phone}"),
        Customer {
            name,
            contact: ContactMethod::Address { city, .. },
            ..
        } => format!("Отправляем письмо {name} в город {city}"),
    }
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(_, 0.0) => Err("Деление на ноль!".to_owned()),
        Operation::Divide(a, b) => Ok(a / b),
    }
}

fn describe_slice(slice: &[&str]) -> String {
    match slice {
        [.., "!"] => "Заканчивается восклицательным знаком".to_owned(),
        ["a", end @ ..] => format!("Начинается с 'a', остаток: {end:?}"),
        all @ [.., last] => format!("Последний элемент {all:?}: {last}"),
        rest => format!("Другой срез: {rest:?}"),
    }
}

fn fizzbuzz_match(x: u32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => "FizzBuzz".to_owned(),
        (0, _) => "Fizz".to_owned(),
        (_, 0) => "Buzz".to_owned(),
        _ => x.to_string(),
    }
}

fn main() {
    move_player(Direction::North);
    println!("Статус: {}", get_status_message(OrderStatus::Shipped));
    println!("Светофор: {}", get_action(TrafficLight::Red));

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Привет!".to_owned()),
        Message::ChangeColor(255, 0, 0),
    ];
    for message in messages {
        println!("{}", process_message(message));
    }

    println!("Площадь круга: {:.2}", calculate_area(Shape::Circle(5.0)));
    println!(
        "Площадь треугольника: {}",
        calculate_area(Shape::Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        })
    );

    println!("Нужен зонт: {}", should_take_umbrella(Weather::Snowy));
    println!("Число 7: {}", analyze_number(7));
    println!("{}", analyze_point((4, 4)));

    let person = Person {
        name: "Иван".to_owned(),
        age: 25,
    };
    println!("{}", describe_person(person));

    let customer = Customer {
        name: "Анна".to_owned(),
        contact: ContactMethod::Email("anna@example.com".to_owned()),
        vip: true,
    };
    println!("{}", contact_customer(customer));

    println!("10 / 4 = {:?}", calculate(Operation::Divide(10.0, 4.0)));
    println!("10 / 0 = {:?}", calculate(Operation::Divide(10.0, 0.0)));

    let optional = Some(42);
    if let Some(value) = optional {
        println!("if let извлёк: {value}");
    }

    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(value) = stack.pop() {
        println!("while let извлёк: {value}");
    }

    println!("{}", describe_slice(&["a", "b", "c"]));
    println!("FizzBuzz для 15: {}", fizzbuzz_match(15));

    let point = (3, 5);
    let (x, y) = point;
    println!("Деструктуризация let: x = {x}, y = {y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_and_traffic_light_are_exhaustive() {
        assert_eq!(get_status_message(OrderStatus::Pending), "В обработке");
        assert_eq!(get_status_message(OrderStatus::Cancelled), "Отменён");
        assert_eq!(get_action(TrafficLight::Green), "Можно ехать");
    }

    #[test]
    fn message_data_is_extracted() {
        assert_eq!(
            process_message(Message::Move { x: 10, y: 20 }),
            "Перемещаемся в точку (10, 20)"
        );
        assert_eq!(
            process_message(Message::ChangeColor(1, 2, 3)),
            "Меняем цвет на RGB(1, 2, 3)"
        );
    }

    #[test]
    fn shape_areas_are_calculated() {
        assert_eq!(calculate_area(Shape::Rectangle(10.0, 20.0)), 200.0);
        assert_eq!(
            calculate_area(Shape::Triangle {
                a: 3.0,
                b: 4.0,
                c: 5.0,
            }),
            6.0
        );
    }

    #[test]
    fn alternatives_ranges_and_guards_work() {
        assert!(should_take_umbrella(Weather::Rainy));
        assert!(!should_take_umbrella(Weather::Sunny));
        assert_eq!(analyze_number(2), "Маленькое число");
        assert_eq!(analyze_number(10), "Среднее число");
        assert_eq!(analyze_point((4, 4)), "На диагонали: (4, 4)");
    }

    #[test]
    fn nested_customer_patterns_work() {
        let customer = Customer {
            name: "Анна".to_owned(),
            contact: ContactMethod::Address {
                street: "Ленина, 1".to_owned(),
                city: "Москва".to_owned(),
            },
            vip: false,
        };

        assert_eq!(
            contact_customer(customer),
            "Отправляем письмо Анна в город Москва"
        );
    }

    #[test]
    fn calculator_handles_division_by_zero() {
        assert_eq!(calculate(Operation::Add(2.0, 3.0)), Ok(5.0));
        assert_eq!(calculate(Operation::Divide(10.0, 4.0)), Ok(2.5));
        assert!(calculate(Operation::Divide(10.0, 0.0)).is_err());
    }

    #[test]
    fn slice_and_tuple_patterns_work() {
        assert_eq!(
            describe_slice(&["a", "b", "c"]),
            "Начинается с 'a', остаток: [\"b\", \"c\"]"
        );
        assert_eq!(fizzbuzz_match(15), "FizzBuzz");
        assert_eq!(fizzbuzz_match(7), "7");
    }
}
