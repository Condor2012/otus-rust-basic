use std::collections::VecDeque;

pub fn process_queue(items: Vec<i32>) -> Vec<i32> {
    let mut queue = VecDeque::new();

    for item in items {
        queue.push_back(item);
    }

    let mut processed = Vec::new();
    while let Some(item) = queue.pop_front() {
        processed.push(item);
    }
    processed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_preserves_fifo_order() {
        assert_eq!(process_queue(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(process_queue(vec![]), Vec::<i32>::new());
    }
}
