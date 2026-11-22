use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex, RwLock};

pub fn assert_send<T: Send>() {}
pub fn assert_send_sync<T: Send + Sync>() {}

pub fn check_thread_safety_bounds() {
    assert_send::<String>();
    assert_send::<RefCell<String>>();

    assert_send_sync::<Arc<String>>();
    assert_send_sync::<AtomicUsize>();
    assert_send_sync::<Mutex<String>>();
    assert_send_sync::<RwLock<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_thread_safe_types_satisfy_bounds() {
        check_thread_safety_bounds();
    }
}
