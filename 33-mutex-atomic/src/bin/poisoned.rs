use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let thread = {
        let mutex_clone = Arc::clone(&mutex);

        thread::spawn(move || {
            let mut data = mutex_clone.lock().unwrap();
            *data += 1;
            // Симуляция паники
            panic!("Oh no!");
        })
    };

    let _ = thread.join();

    println!("Is poisoned: {}", mutex.is_poisoned());

    match mutex.lock() {
        Ok(data) => {
            println!("Data: {}", *data);
        }
        Err(mut poisoned) => {
            println!("Poisoned: {:?}", poisoned);
            **poisoned.get_mut() = 0;
            mutex.clear_poison();
        }
    };

    println!("Is poisoned: {}", mutex.is_poisoned());
    println!("Lock: {:?}", mutex.lock());
}
