use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let data = Arc::new(Mutex::new(0));

    let thread = {
        let counter = Arc::clone(&counter);
        let data = Arc::clone(&data);

        thread::spawn(move || {
            println!("[dir]: before num lock");
            let mut counter = counter.lock().unwrap();
            println!("[dir]: locked num");

            thread::sleep(std::time::Duration::from_millis(10));

            println!("[dir]: before data lock");
            let mut data = data.lock().unwrap();
            println!("[dir]: locked data");

            *counter += 1;
            *data += 10;

            println!("[dir]: unlocked");
        })
    };

    let reversed = {
        let counter = Arc::clone(&counter);
        let data = Arc::clone(&data);

        thread::spawn(move || {
            println!("[rev]: before data lock");
            let mut data = data.lock().unwrap();
            println!("[rev]: locked data");

            thread::sleep(std::time::Duration::from_millis(10));

            println!("[rev]: before num lock");
            let mut counter = counter.lock().unwrap();
            println!("[rev]: locked num");

            *counter += 1;
            *data += 10;

            println!("[rev]: unlocked");
        })
    };

    thread.join().unwrap();
    reversed.join().unwrap();

    println!("Counter result: {}", *counter.lock().unwrap());
    println!("Data result: {}", *data.lock().unwrap());
}
