use threads_mutex_atomic_practice::{
    atomic_i32_counter, atomic_usize_counter, bank_account, hello_threads, mutex_counter,
    parallel_sum, shared_counter, sum_in_threads,
};

fn main() {
    for message in hello_threads::run() {
        println!("{message}");
    }
    println!("Все потоки завершены!");

    println!("Общая сумма: {}", sum_in_threads::run());

    for (thread_id, value) in shared_counter::run() {
        println!("Поток {thread_id} видит: {value}");
    }

    println!("Счётчик с Mutex: {}", mutex_counter::run());
    println!("Счётчик AtomicI32: {}", atomic_i32_counter::run());
    println!("Параллельная сумма: {}", parallel_sum::run());
    println!("Счётчик AtomicUsize: {}", atomic_usize_counter::run());
    println!("Итоговый баланс: {}", bank_account::run());
}
