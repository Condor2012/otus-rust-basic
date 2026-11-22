use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let counter = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for i in 1..=5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(i));
            println!("[r{i}]: Before read lock");
            let value = counter.read().unwrap();
            println!("[r{i}]: read-locked");
            thread::sleep(std::time::Duration::from_millis(1000));
            println!("[r{i}]: Read value: {}", *value);
        });
        handles.push(handle);
    }

    for i in 1..=2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(2));
            println!("[w{i}]: Before write lock");
            let mut value = counter.write().unwrap();
            println!("[w{i}]: write-locked");
            *value += 10;
            println!("[w{i}] Changed value: {}", *value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *counter.read().unwrap());
}
