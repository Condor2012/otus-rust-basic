use std::sync::Arc;
use std::thread;

pub fn spawn_and_join() -> i32 {
    let handle = thread::spawn(|| 42);
    handle.join().unwrap()
}

pub fn scoped_shared_reference() -> String {
    let init_cmd = String::from("show database");
    let cmd_ref = &init_cmd;

    thread::scope(|scope| {
        let handle = scope.spawn(|| perform(cmd_ref));
        handle.join().unwrap()
    })
}

pub fn arc_move_to_thread() -> String {
    let init_cmd = String::from("show database");
    let cmd_arc = Arc::new(init_cmd);
    let cmd_ref = Arc::clone(&cmd_arc);

    let handle = thread::spawn(move || perform(cmd_ref.as_str()));
    handle.join().unwrap()
}

fn perform(command: &str) -> String {
    format!("Выполняется: {command}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_returns_thread_result() {
        assert_eq!(spawn_and_join(), 42);
    }

    #[test]
    fn scoped_thread_can_borrow_local_string() {
        assert_eq!(scoped_shared_reference(), "Выполняется: show database");
    }

    #[test]
    fn arc_owns_data_for_spawned_thread() {
        assert_eq!(arc_move_to_thread(), "Выполняется: show database");
    }
}
