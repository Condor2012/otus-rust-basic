/// Указатель на функцию с двумя аргументами i32 и результатом i32.
pub type BinaryOperation = fn(i32, i32) -> i32;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Принимает как именованную функцию, так и лямбду без захвата окружения.
pub fn apply(a: i32, b: i32, operation: BinaryOperation) -> i32 {
    operation(a, b)
}

#[cfg(test)]
mod tests {
    use super::{add, apply, multiply};

    #[test]
    fn accepts_named_functions() {
        assert_eq!(apply(2, 3, add), 5);
        assert_eq!(apply(2, 3, multiply), 6);
    }

    #[test]
    fn accepts_non_capturing_closure() {
        assert_eq!(apply(7, 2, |a, b| a - b), 5);
    }
}
