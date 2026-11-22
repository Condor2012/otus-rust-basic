use lesson_14_structs_enums_pattern_matching_practice::patterns::{
    Asset, Expression, HttpStatus, grade_match, is_error, process, traffic_light,
};
use lesson_14_structs_enums_pattern_matching_practice::thermostat::{Mode, Thermostat};

fn main() {
    println!("Красный свет: {}", traffic_light("red"));
    println!("HTTP 404 — ошибка: {}", is_error(HttpStatus::NotFound));
    println!("Some(2) + Some(3) = {}", process(Some(2), Some(3)));
    println!("Оценка для 85: {}", grade_match(85));

    let expression = Expression::Add(
        Box::new(Expression::Number(20.0)),
        Box::new(Expression::Number(22.0)),
    );
    println!("Значение выражения: {:?}", expression.eval());

    let asset = Asset::Stock {
        symbol: "RUST".to_owned(),
        shares: 10,
        price: 25.0,
    };
    println!("{}: {}", asset.description(), asset.value());

    let mut thermostat = Thermostat::new(Mode::Heating, 20, 25, 2);
    println!("\nНачальное состояние:\n{}", thermostat.info());

    thermostat.turn_on();
    for temperature in [18, 19, 20, 18] {
        thermostat.update_temperature(temperature);
        println!("\n{}", thermostat.info());
    }
    println!("\nКоличество активаций: {}", thermostat.activation_count());

    thermostat.turn_off();
    println!("\nПосле выключения:\n{}", thermostat.info());
}
