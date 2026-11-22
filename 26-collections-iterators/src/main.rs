use lesson_26_collections_iterators::collections::{
    character_frequencies, copy_from_slice, earth_distance, planet_distances,
};
use lesson_26_collections_iterators::custom_iterator::{MyType, collect_with_into_iterator};
use lesson_26_collections_iterators::from_iterator::{
    collect_options, collect_results, filter_letters, ordered_unique_items, unique_chars,
};
use lesson_26_collections_iterators::iterator_basics::{
    consumers, double_in_place, enumerate_skip_take, even_doubles, first_five_from_infinite_range,
    manual_next,
};
use lesson_26_collections_iterators::laziness::parity;

fn main() {
    println!("copy_from_slice: {:?}", copy_from_slice(&[1, 2, 3]));
    println!("Частоты символов: {:?}", character_frequencies("abca"));
    println!(
        "Расстояние до Земли: {:?}",
        earth_distance(&planet_distances())
    );

    println!("\nРучные вызовы next: {:?}", manual_next());
    println!("map + filter: {:?}", even_doubles());
    println!(
        "take из бесконечного диапазона: {:?}",
        first_five_from_infinite_range()
    );
    println!(
        "step_by + enumerate + skip + take: {:?}",
        enumerate_skip_take()
    );
    println!("Потребители итератора: {:?}", consumers(&[1, 2, 3, 4]));

    let mut values = vec![1, 2, 3];
    double_in_place(&mut values);
    println!("iter_mut: {values:?}");
    println!(
        "IntoIterator: {:?}",
        collect_with_into_iterator(vec![1, 2, 3])
    );

    let custom = MyType {
        items: vec!["one".to_string(), "two".to_string()],
    };
    println!(
        "Собственный итератор: {:?}",
        custom.custom_iter().collect::<Vec<_>>()
    );

    println!("\nТолько буквы: {}", filter_letters("a1-b2!"));
    println!("Уникальные символы: {:?}", unique_chars("abca"));
    println!(
        "Упорядоченные уникальные: {:?}",
        ordered_unique_items(vec![3, 1, 3, 2])
    );
    println!("Collect Result: {:?}", collect_results(vec![Ok(1), Ok(2)]));
    println!(
        "Collect Option: {:?}",
        collect_options(vec![Some(1), Some(2)])
    );
    println!("Чётность: {:?}", parity(&[1, 2, 3]));
}
