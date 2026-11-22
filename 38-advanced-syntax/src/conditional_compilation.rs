#[cfg(target_os = "linux")]
pub fn system_info() -> &'static str {
    "Linux system"
}

#[cfg(target_os = "windows")]
pub fn system_info() -> &'static str {
    "Windows system"
}

#[cfg(target_os = "macos")]
pub fn system_info() -> &'static str {
    "macOS system"
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn system_info() -> &'static str {
    "Other system"
}

#[cfg(debug_assertions)]
pub fn build_mode() -> &'static str {
    "debug"
}

#[cfg(not(debug_assertions))]
pub fn build_mode() -> &'static str {
    "release"
}

#[cfg(test)]
mod tests {
    use super::{build_mode, system_info};

    #[test]
    fn selects_exactly_one_implementation() {
        assert!(!system_info().is_empty());
        assert!(matches!(build_mode(), "debug" | "release"));
    }
}
