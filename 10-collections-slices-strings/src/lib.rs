mod choose_elem;
mod get_nth;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_elem() {
        let mut v = vec![1, 2, 3, 4, 5];
        let elem = choose_elem::choose_elem(&mut v, 2);
        *elem += 10;
        assert_eq!(v, vec![1, 2, 13, 4, 5]);
    }

    #[test]
    fn test_get_nth() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_nth::get_nth(&v, 2), Some(&3));
        assert_eq!(get_nth::get_nth(&v, 10), None);
    }
}
