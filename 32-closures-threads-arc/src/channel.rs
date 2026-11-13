use std::{result, thread};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..5 {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                tx.send(format!("Успех {}", i)).unwrap();
            } else {
                eprintln!("Ошибка в потоке: {}", i)
            }
        });

        handles.push(handle);
    }

    drop(tx);

    let results: Vec<_> = rx.iter().collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Финальный результат: {:?}", results);
}
