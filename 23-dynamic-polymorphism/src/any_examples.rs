use std::any::{Any, TypeId};

pub fn demo_values() -> Vec<Box<dyn Any>> {
    vec![
        Box::new(42_u32),
        Box::new(1_f32),
        Box::new("abc".to_string()),
    ]
}

pub fn describe(value: &dyn Any) -> String {
    if let Some(number) = value.downcast_ref::<u32>() {
        format!("u32: {number}")
    } else if let Some(number) = value.downcast_ref::<f32>() {
        format!("f32: {number}")
    } else if let Some(text) = value.downcast_ref::<String>() {
        format!("String: {text}")
    } else {
        "неизвестный тип".to_string()
    }
}

pub fn same_type<T: 'static, U: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<U>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downcast_recovers_concrete_types() {
        let descriptions = demo_values()
            .iter()
            .map(|value| describe(value.as_ref()))
            .collect::<Vec<_>>();

        assert_eq!(descriptions, ["u32: 42", "f32: 1", "String: abc"]);
    }

    #[test]
    fn failed_downcast_returns_none() {
        let value: Box<dyn Any> = Box::new(42_u32);

        assert!(value.downcast_ref::<String>().is_none());
        assert_eq!(value.downcast_ref::<u32>(), Some(&42));
    }

    #[test]
    fn type_id_compares_static_types() {
        assert!(same_type::<u32, u32>());
        assert!(!same_type::<u32, String>());
    }
}
