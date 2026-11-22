use lesson_27_collections_iterators_practice::frequency::count_frequencies;
use lesson_27_collections_iterators_practice::people::{Person, sort_by_age_descending};
use lesson_27_collections_iterators_practice::queue::process_queue;
use lesson_27_collections_iterators_practice::strings::{find_long_string, group_by_length};
use lesson_27_collections_iterators_practice::unique::{
    first_unique_element, merge_unique, unique_elements,
};
use lesson_27_collections_iterators_practice::vector_practice::{
    filter_and_uppercase, find_min, is_palindrome, process_numbers, remove_all_occurrences,
    sum_elements,
};

fn main() {
    println!(
        "1. Частоты: {:?}",
        count_frequencies(vec![1, 2, 3, 2, 1, 3, 4, 5, 4, 1])
    );
    println!(
        "2. Фильтр и верхний регистр: {:?}",
        filter_and_uppercase(vec!["rust", "is", "awesome", "and", "fast"])
    );
    println!("2.1. Минимум: {:?}", find_min(vec![5, 3, 9, 1, 4]));
    println!("2.2. Сумма: {}", sum_elements(vec![1, 2, 3, 4, 5]));
    println!(
        "2.3. Без двоек: {:?}",
        remove_all_occurrences(vec![1, 2, 3, 2, 4, 2, 5], 2)
    );
    println!("2.4. Палиндром: {}", is_palindrome(vec![1, 2, 3, 2, 1]));
    println!("3. Очередь FIFO: {:?}", process_queue(vec![1, 2, 3]));
    println!(
        "4. Уникальные (порядок не гарантирован): {:?}",
        unique_elements(vec![1, 2, 3, 2, 1, 4, 5])
    );

    let people = vec![
        Person::new("Alice", 30),
        Person::new("Bob", 25),
        Person::new("Charlie", 35),
    ];
    println!("5. По возрасту: {:?}", sort_by_age_descending(people));
    println!(
        "6. Объединение без дубликатов (порядок не гарантирован): {:?}",
        merge_unique(vec![1, 2, 3], vec![3, 4, 5])
    );
    println!(
        "7. Первый уникальный: {:?}",
        first_unique_element(vec![1, 2, 3, 2, 1, 4, 5])
    );

    let strings = vec![
        "apple".to_string(),
        "banana".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
    ];
    println!("8. По длине: {:?}", group_by_length(strings));
    println!(
        "9. Чётные × 2: {:?}",
        process_numbers(vec![1, 2, 3, 4, 5, 6])
    );

    let words = vec![
        "cat".to_string(),
        "tiger".to_string(),
        "elephant".to_string(),
    ];
    println!("10. Первая строка длиннее 5: {}", find_long_string(words));
}
