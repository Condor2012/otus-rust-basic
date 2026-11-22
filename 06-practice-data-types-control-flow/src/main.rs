fn fizzbuzz(num: u32) -> String {
    if num.is_multiple_of(15) {
        "FizzBuzz".to_owned()
    } else if num.is_multiple_of(3) {
        "Fizz".to_owned()
    } else if num.is_multiple_of(5) {
        "Buzz".to_owned()
    } else {
        num.to_string()
    }
}

fn fizzbuzz_list(n: u32) -> Vec<String> {
    let mut result = Vec::new();

    for num in 1..=n {
        result.push(fizzbuzz(num));
    }

    result
}

fn digit_product(n: u32) -> u8 {
    if n == 0 {
        return 0;
    }

    let mut current = n;

    while current >= 10 {
        let mut product = 1_u32;
        let mut rest = current;

        while rest > 0 {
            let digit = rest % 10;
            if digit != 0 {
                product *= digit;
            }
            rest /= 10;
        }

        current = product;
    }

    current as u8
}

fn fib(n: u32) -> u32 {
    let mut previous = 0_u32;
    let mut current = 1_u32;

    for _ in 0..n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    previous
}

fn fib_sequence(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();

    for index in 0..=n {
        sequence.push(fib(index));
    }

    sequence
}

fn uniq_digit(s: &str) -> u8 {
    find_uniq_digit(s).expect("в строке должна быть одна уникальная цифра")
}

fn find_uniq_digit(s: &str) -> Option<u8> {
    let mut counts = [0_u32; 10];

    for byte in s.bytes() {
        if byte.is_ascii_digit() {
            let index = usize::from(byte - b'0');
            counts[index] += 1;
        }
    }

    for (digit, count) in counts.iter().enumerate() {
        if *count == 1 {
            return Some(digit as u8);
        }
    }

    None
}

fn missing_num(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = nums.iter().sum();

    expected_sum - actual_sum
}

fn validate_paren(s: &str) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                let expected = match ch {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => unreachable!(),
                };

                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}

fn main() {
    println!("FizzBuzz от 1 до 15:");
    for value in fizzbuzz_list(15) {
        println!("{value}");
    }

    let number = 123_454_321;
    println!(
        "\nОднозначный результат произведения цифр {number}: {}",
        digit_product(number)
    );

    println!("\nФибоначчи до индекса 10: {:?}", fib_sequence(10));

    let digits = "123454321";
    println!("\nУникальная цифра в {digits}: {}", uniq_digit(digits));
    println!(
        "Уникальная цифра в произвольной строке: {:?}",
        find_uniq_digit("abc123454321")
    );

    let nums = [0, 4, 2, 5, 3, 6];
    println!("\nОтсутствующее число в {nums:?}: {}", missing_num(&nums));

    let parentheses = "({[]()})";
    println!(
        "\nСтрока {parentheses} корректна: {}",
        validate_paren(parentheses)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }

    #[test]
    fn fizzbuzz_list_works() {
        let expected: Vec<String> = ["1", "2", "Fizz", "4", "Buzz"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(fizzbuzz_list(5), expected);
        assert!(fizzbuzz_list(0).is_empty());
    }

    #[test]
    fn digit_product_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
        assert_eq!(digit_product(1000), 1);
    }

    #[test]
    fn fibonacci_works() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(7), 13);
        assert_eq!(fib_sequence(7), vec![0, 1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(fib_sequence(0), vec![0]);
    }

    #[test]
    fn uniq_digit_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }

    #[test]
    fn optional_uniq_digit_works() {
        assert_eq!(find_uniq_digit("abc123454321"), Some(5));
        assert_eq!(find_uniq_digit("abc"), None);
        assert_eq!(find_uniq_digit("1122"), None);
    }

    #[test]
    fn missing_num_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
        assert_eq!(missing_num(&[]), 0);
    }

    #[test]
    fn validate_paren_works() {
        assert!(validate_paren("()"));
        assert!(validate_paren("()[]{}"));
        assert!(validate_paren("({[]()})"));
        assert!(validate_paren(""));
        assert!(!validate_paren("(}"));
        assert!(!validate_paren("()]"));
        assert!(!validate_paren("(){"));
        assert!(!validate_paren("]"));
    }
}
