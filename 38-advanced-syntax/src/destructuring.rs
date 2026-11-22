pub struct Address {
    pub street: String,
    pub city: String,
}

pub struct Person {
    pub name: String,
    pub address: Address,
}

pub fn person_parts(person: Person) -> (String, String, String) {
    let Person {
        name,
        address: Address { street, city },
    } = person;
    (name, street, city)
}

pub fn tuple_edges(values: (i32, i32, i32, i32)) -> (i32, i32) {
    let (first, .., last) = values;
    (first, last)
}

pub fn array_edges(values: &[i32]) -> Option<(i32, i32)> {
    match values {
        [first, .., last] => Some((*first, *last)),
        [] => None,
    }
}

pub fn describe_nested(value: Option<Result<i32, String>>) -> String {
    match value {
        Some(Ok(number)) => format!("Успех: {number}"),
        Some(Err(error)) => format!("Ошибка: {error}"),
        None => "Нет значения".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructures_nested_structs() {
        let person = Person {
            name: "Анна".into(),
            address: Address {
                street: "Ленина".into(),
                city: "Казань".into(),
            },
        };
        assert_eq!(person_parts(person), ("Анна".into(), "Ленина".into(), "Казань".into()));
    }

    #[test]
    fn ignores_middle_values() {
        assert_eq!(tuple_edges((1, 2, 3, 4)), (1, 4));
        assert_eq!(array_edges(&[1, 2, 3]), Some((1, 3)));
        assert_eq!(array_edges(&[]), None);
    }

    #[test]
    fn matches_option_result() {
        assert_eq!(describe_nested(Some(Ok(42))), "Успех: 42");
        assert_eq!(describe_nested(Some(Err("ошибка".into()))), "Ошибка: ошибка");
        assert_eq!(describe_nested(None), "Нет значения");
    }
}
