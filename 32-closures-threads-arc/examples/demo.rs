use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let val = 45;

    let hand = thread::spawn(move || {
        let new_val = val + 10;
        tx.send(new_val).unwrap();
    });

    hand.join().unwrap();

    let res = rx.recv().unwrap();
    println!("{}", res);
}
