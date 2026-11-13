pub fn choose_elem(tuple: &mut (i32, i32), pick_second: bool) -> &mut i32 {
    if pick_second {
        &mut tuple.1
    } else {
        &mut tuple.0
    }

}

pub fn get_nth(slice: &mut [i32], n: usize) -> &mut i32 {
    &mut slice[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_elem() {
        let mut pair = (10, 20);

        let first = choose_elem(&mut pair, false);
        *first = 99;
        assert_eq!(pair, (99, 20));

        let second = choose_elem(&mut pair, true);
        *second = 88;
        assert_eq!(pair, (99, 88));

    }

    #[test]
    fn test_get_nth_element () {
        let mut array = [1, 2, 3, 4, 5];

        let element = get_nth(&mut array[..3], 2);
        *element = 100;

        assert_eq!(array, [1, 2, 100, 4, 5])
    }
}
