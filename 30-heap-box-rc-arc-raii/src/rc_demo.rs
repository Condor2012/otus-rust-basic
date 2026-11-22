use std::rc::Rc;

pub fn rc_counts() -> (usize, usize, usize) {
    let value = Rc::new(String::from("общие данные"));
    let initial = Rc::strong_count(&value);

    let during_scope = {
        let _first_clone = Rc::clone(&value);
        let _second_clone = Rc::clone(&value);
        Rc::strong_count(&value)
    };

    let after_scope = Rc::strong_count(&value);
    (initial, during_scope, after_scope)
}

pub fn weak_upgrade_states() -> (bool, bool) {
    let value = Rc::new(42);
    let weak = Rc::downgrade(&value);

    let while_alive = weak.upgrade().is_some();
    drop(value);
    let after_drop = weak.upgrade().is_some();

    (while_alive, after_drop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_increment_and_drops_decrement_count() {
        assert_eq!(rc_counts(), (1, 3, 1));
    }

    #[test]
    fn weak_does_not_keep_value_alive() {
        assert_eq!(weak_upgrade_states(), (true, false));
    }
}
