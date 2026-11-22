use std::sync::mpsc;
use std::thread;

pub fn async_channel(values: Vec<i32>) -> Vec<i32> {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        for value in values {
            sender.send(value).unwrap();
        }
    });

    let received = receiver.into_iter().collect();
    handle.join().unwrap();
    received
}

pub fn sync_channel(values: Vec<i32>) -> Vec<i32> {
    let (sender, receiver) = mpsc::sync_channel(2);

    let handle = thread::spawn(move || {
        for value in values {
            sender.send(value).unwrap();
        }
    });

    let received = receiver.into_iter().collect();
    handle.join().unwrap();
    received
}

pub fn multiple_producers() -> Vec<i32> {
    let (sender, receiver) = mpsc::channel();
    let first_sender = sender.clone();
    let second_sender = sender.clone();

    let first = thread::spawn(move || first_sender.send(1).unwrap());
    let second = thread::spawn(move || second_sender.send(2).unwrap());
    drop(sender);

    let mut received = receiver.into_iter().collect::<Vec<_>>();
    first.join().unwrap();
    second.join().unwrap();
    received.sort_unstable();
    received
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_and_sync_channels_transfer_owned_values() {
        assert_eq!(async_channel(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(sync_channel(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn cloned_senders_feed_one_receiver() {
        assert_eq!(multiple_producers(), vec![1, 2]);
    }
}
