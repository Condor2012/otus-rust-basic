use std::cell::RefCell;

pub fn perform(command: &str) -> String {
    format!("Выполняется: {command}")
}

pub fn change_command() -> String {
    let init_cmd = String::from("show database");
    let cmd = RefCell::new(init_cmd);
    *cmd.borrow_mut() = String::from("drop garbage");

    perform(&cmd.borrow())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refcell_mutates_value_through_shared_reference() {
        assert_eq!(change_command(), "Выполняется: drop garbage");
    }
}
