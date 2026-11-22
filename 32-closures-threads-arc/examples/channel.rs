use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for worker_id in 0..5 {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            if worker_id % 2 == 0 {
                tx.send(format!("Успех {worker_id}")).unwrap();
            } else {
                eprintln!("Поток {worker_id}: значение не отправлено");
            }
        });

        handles.push(handle);
    }

    drop(tx);

    let results: Vec<_> = rx.iter().collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Финальный результат: {results:?}");
}
