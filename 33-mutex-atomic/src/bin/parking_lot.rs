fn compare_size<T>() {
    println!("std size: {}", size_of::<std::sync::Mutex<T>>());
    println!("std align: {}", align_of::<std::sync::Mutex<T>>());
    println!("parking_lot size: {}", size_of::<parking_lot::Mutex<T>>());
    println!("parking_lot align: {}", align_of::<parking_lot::Mutex<T>>());
}

fn main() {
    compare_size::<u8>();
}
