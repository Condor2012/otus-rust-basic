pub fn traffic_light(color: &str) -> &str {
    match color {
        "red" => "Stop",
        "yellow" => "Slow down",
        "green" => "Go",
        _ => "Unknown color",
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpStatus {
    Ok,
    NotFound,
    ServerError,
    BadRequest,
    Unauthorized,
}

pub fn is_error(status: HttpStatus) -> bool {
    match status {
        HttpStatus::Ok => false,
        HttpStatus::NotFound
        | HttpStatus::ServerError
        | HttpStatus::BadRequest
        | HttpStatus::Unauthorized => true,
    }
}

pub fn process(opt1: Option<i32>, opt2: Option<i32>) -> i32 {
    match (opt1, opt2) {
        (Some(x), Some(y)) => x + y,
        (Some(x), None) => x,
        (None, Some(y)) => y,
        (None, None) => 0,
    }
}

pub fn grade_if(score: u8) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'F'
    }
}

pub fn grade_match(score: u8) -> char {
    match score {
        90..=u8::MAX => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    }
}

pub struct Foo {
    a: u32,
    b: String,
}

impl Foo {
    pub fn new(a: u32, b: &str) -> Self {
        Self {
            a,
            b: b.to_owned(),
        }
    }
}

pub fn foo(f: Foo) -> String {
    let matched = match &f {
        Foo { a: 0, b } => format!("str: {b}"),
        Foo { a, .. } => format!("num: {a}"),
    };

    bar(f);
    matched
}

pub fn bar(f: Foo) {
    println!("Foo всё ещё доступен: a = {}, b = {}", f.a, f.b);
}

pub enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Expression::Number(value) => Ok(*value),
            Expression::Add(left, right) => Ok(left.eval()? + right.eval()?),
            Expression::Subtract(left, right) => Ok(left.eval()? - right.eval()?),
            Expression::Multiply(left, right) => Ok(left.eval()? * right.eval()?),
            Expression::Divide(left, right) => {
                let divisor = right.eval()?;
                if divisor == 0.0 {
                    Err("Деление на ноль".to_owned())
                } else {
                    Ok(left.eval()? / divisor)
                }
            }
        }
    }
}

pub enum Asset {
    Cash(f64),
    Stock {
        symbol: String,
        shares: u32,
        price: f64,
    },
    RealEstate {
        address: String,
        value: f64,
    },
}

impl Asset {
    pub fn value(&self) -> f64 {
        match self {
            Asset::Cash(value) => *value,
            Asset::Stock { shares, price, .. } => f64::from(*shares) * price,
            Asset::RealEstate { value, .. } => *value,
        }
    }

    pub fn description(&self) -> String {
        match self {
            Asset::Cash(_) => "Денежные средства".to_owned(),
            Asset::Stock { symbol, .. } => format!("Акции {symbol}"),
            Asset::RealEstate { address, .. } => format!("Недвижимость: {address}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traffic_light_handles_all_inputs() {
        assert_eq!(traffic_light("red"), "Stop");
        assert_eq!(traffic_light("yellow"), "Slow down");
        assert_eq!(traffic_light("green"), "Go");
        assert_eq!(traffic_light("blue"), "Unknown color");
    }

    #[test]
    fn http_errors_are_classified() {
        assert!(!is_error(HttpStatus::Ok));
        assert!(is_error(HttpStatus::NotFound));
        assert!(is_error(HttpStatus::Unauthorized));
    }

    #[test]
    fn options_are_summed() {
        assert_eq!(process(Some(2), Some(3)), 5);
        assert_eq!(process(Some(2), None), 2);
        assert_eq!(process(None, Some(3)), 3);
        assert_eq!(process(None, None), 0);
    }

    #[test]
    fn grading_implementations_match() {
        for score in [0, 69, 70, 79, 80, 89, 90, 100, u8::MAX] {
            assert_eq!(grade_if(score), grade_match(score));
        }
    }

    #[test]
    fn foo_is_not_partially_moved() {
        assert_eq!(foo(Foo::new(0, "hello")), "str: hello");
        assert_eq!(foo(Foo::new(42, "world")), "num: 42");
    }

    #[test]
    fn expression_is_evaluated_recursively() {
        let expression = Expression::Multiply(
            Box::new(Expression::Add(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Number(4.0)),
        );

        assert_eq!(expression.eval(), Ok(20.0));
        assert!(
            Expression::Divide(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(0.0)),
            )
            .eval()
            .is_err()
        );
    }

    #[test]
    fn assets_expose_description_and_value() {
        let stock = Asset::Stock {
            symbol: "RUST".to_owned(),
            shares: 5,
            price: 20.0,
        };
        assert_eq!(stock.value(), 100.0);
        assert_eq!(stock.description(), "Акции RUST");
        assert_eq!(Asset::Cash(50.0).value(), 50.0);
    }
}
