pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub fn describe_message(message: &Message) -> String {
    match message {
        Message::Quit => "Выход".into(),
        Message::Move { x, y } => format!("Перемещение в ({x}, {y})"),
        Message::Write(text) => format!("Текст: {text}"),
    }
}

pub fn range_name(value: i32) -> String {
    match value {
        x @ 1..=10 => format!("{x} в диапазоне 1–10"),
        x @ 11..=20 => format!("{x} в диапазоне 11–20"),
        _ => "Вне диапазона".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, describe_message, range_name};

    #[test]
    fn matches_every_variant() {
        assert_eq!(describe_message(&Message::Quit), "Выход");
        assert_eq!(
            describe_message(&Message::Move { x: 2, y: 3 }),
            "Перемещение в (2, 3)"
        );
        assert_eq!(
            describe_message(&Message::Write("Rust".into())),
            "Текст: Rust"
        );
    }

    #[test]
    fn binds_a_matched_range() {
        assert_eq!(range_name(7), "7 в диапазоне 1–10");
        assert_eq!(range_name(15), "15 в диапазоне 11–20");
        assert_eq!(range_name(30), "Вне диапазона");
    }
}
