mod choose_elem;
mod get_nth;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = choose_elem::choose_elem(&mut vec, 2);
    *elem += 10;
    println!("{vec:?}");

    let mut vec_2 = vec![1, 2, 3, 4, 5];
    let elem_2 = choose_elem::choose_elem(&mut vec_2, 3);
    *elem_2 += 20;
    println!("{vec_2:?}");

    let opt = get_nth::get_nth(&vec, 10);
    println!("{opt:?}");
}
