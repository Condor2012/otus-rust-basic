use std::sync::Arc;
use std::thread;

pub fn arc_between_threads() -> (String, String, usize) {
    let value = Arc::new(String::from("общие данные"));

    let first = Arc::clone(&value);
    let second = Arc::clone(&value);

    let first_handle = thread::spawn(move || (*first).clone());
    let second_handle = thread::spawn(move || (*second).clone());

    let first_result = first_handle.join().unwrap();
    let second_result = second_handle.join().unwrap();
    let remaining = Arc::strong_count(&value);

    (first_result, second_result, remaining)
}

pub fn arc_weak_upgrade_states() -> (bool, bool) {
    let value = Arc::new(42);
    let weak = Arc::downgrade(&value);

    let while_alive = weak.upgrade().is_some();
    drop(value);
    let after_drop = weak.upgrade().is_some();

    (while_alive, after_drop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_shares_owned_data_between_threads() {
        let (first, second, remaining) = arc_between_threads();

        assert_eq!(first, "общие данные");
        assert_eq!(second, "общие данные");
        assert_eq!(remaining, 1);
    }

    #[test]
    fn arc_weak_upgrades_only_while_strong_owner_exists() {
        assert_eq!(arc_weak_upgrade_states(), (true, false));
    }
}
