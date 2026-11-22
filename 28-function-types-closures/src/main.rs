use function_types_closures::{closure_traits, function_pointers};

fn main() {
    println!(
        "Указатель на функцию add: {}",
        function_pointers::apply(2, 3, function_pointers::add)
    );
    println!(
        "Лямбда без захвата: {}",
        function_pointers::apply(7, 2, |a, b| a - b)
    );

    let (fn_result, fn_mut_result, fn_once_result) = closure_traits::examples();
    println!("Fn: {fn_result}");
    println!("FnMut: {fn_mut_result}");
    println!("FnOnce: {fn_once_result}");
}
