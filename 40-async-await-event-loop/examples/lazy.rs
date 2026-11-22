use std::cell::Cell;
use async_await_event_loop::expensive_operation;

#[tokio::main]
async fn main() {
    let started = Cell::new(false);
    let future = expensive_operation(&started);

    println!("До await started = {started:?}", started = started.get());
    drop(future);
    println!("После drop started = {started:?}", started = started.get());

    let result = expensive_operation(&started).await;
    println!("После await: result = {result}, started = {}", started.get());
}
