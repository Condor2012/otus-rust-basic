use lesson_31_threads_send_sync::channels::{async_channel, multiple_producers, sync_channel};
use lesson_31_threads_send_sync::interior_mutability::change_command;
use lesson_31_threads_send_sync::markers::check_thread_safety_bounds;
use lesson_31_threads_send_sync::synchronization::{
    atomic_counter, barrier_example, mutex_counter, rwlock_example,
};
use lesson_31_threads_send_sync::threads::{
    arc_move_to_thread, scoped_shared_reference, spawn_and_join,
};

fn main() {
    println!("spawn + join: {}", spawn_and_join());
    println!("thread::scope и &String: {}", scoped_shared_reference());
    println!("Arc + move: {}", arc_move_to_thread());
    println!("RefCell в одном потоке: {}", change_command());

    check_thread_safety_bounds();
    println!("Проверки Send + Sync выполнены на этапе компиляции");

    println!("Atomic: {}", atomic_counter());
    println!("Mutex: {}", mutex_counter());
    println!("RwLock: {:?}", rwlock_example());
    barrier_example();
    println!("Barrier: оба потока прошли точку синхронизации");

    println!("mpsc::channel: {:?}", async_channel(vec![1, 2, 3]));
    println!("mpsc::sync_channel: {:?}", sync_channel(vec![1, 2, 3]));
    println!("Несколько отправителей: {:?}", multiple_producers());
}
