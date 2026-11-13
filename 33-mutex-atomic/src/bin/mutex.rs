use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug, Default)]
struct Counter(usize);

impl Counter {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

fn expensive_calc() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let counter = Arc::new(Mutex::new(Counter::default()));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            if let Ok(mut cnt) = counter.try_lock() {
                cnt.inc();
            }
            {
                let mut cnt = counter.lock().unwrap();
                cnt.inc();
            }
            expensive_calc();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *counter.lock().unwrap());
}
