use lesson_25_collections_big_o_vec_hashmap::complexity::{
    all_pairs, binary_search, first, linear_search, sorted_copy,
};
use lesson_25_collections_big_o_vec_hashmap::hashmap_demo::{score_operations, sorted_scores};
use lesson_25_collections_big_o_vec_hashmap::vector_demo::{
    capacity_survives_clear, create_with_macro, create_with_push, edit_and_sort,
};

fn main() {
    let values = [1, 3, 5, 7];
    println!("O(1), первый элемент: {:?}", first(&values));
    println!("O(log n), поиск 5: {:?}", binary_search(&values, 5));
    println!("O(n), поиск 7: {:?}", linear_search(&values, 7));
    println!("O(n log n), сортировка [3, 1, 2]: {:?}", sorted_copy(&[3, 1, 2]));
    println!("O(n²), все пары [1, 2]: {:?}", all_pairs(&[1, 2]));

    println!("\nVec::new + push: {:?}", create_with_push());
    println!("vec![1, 2, 3]: {:?}", create_with_macro());
    println!(
        "Ёмкость до clear, длина после, ёмкость после: {:?}",
        capacity_survives_clear()
    );
    println!("Vec после изменений и сортировки: {:?}", edit_and_sort());

    let scores = score_operations();
    println!("\nHashMap после операций: {:?}", sorted_scores(&scores));
    println!("Длина: {}, ёмкость: {}", scores.len(), scores.capacity());
}
