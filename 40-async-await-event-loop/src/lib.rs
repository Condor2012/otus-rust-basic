use std::cell::Cell;

/// Тело async-функции не начинает выполняться до первого poll/await.
pub async fn expensive_operation(started: &Cell<bool>) -> i32 {
    started.set(true);
    42
}

/// Оценка памяти на потоки из расчёта C10K на слайдах.
pub fn thread_memory_estimate(connections: usize) -> (usize, usize) {
    let stacks_mb = connections * 2;
    let kernel_mb = connections * 20 / 1_000;
    (stacks_mb, kernel_mb)
}

#[cfg(test)]
mod tests {
    use super::{expensive_operation, thread_memory_estimate};
    use std::cell::Cell;

    #[tokio::test]
    async fn future_is_lazy_until_awaited() {
        let started = Cell::new(false);
        let future = expensive_operation(&started);
        assert!(!started.get());
        drop(future);
        assert!(!started.get());

        let result = expensive_operation(&started).await;
        assert_eq!(result, 42);
        assert!(started.get());
    }

    #[test]
    fn estimates_c10k_memory() {
        assert_eq!(thread_memory_estimate(10_000), (20_000, 200));
    }
}
