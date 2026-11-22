# Занятие 27. Практика: коллекции и итераторы

Решения перенесены из ссылок на Rust Playground, указанных на слайдах с заданиями. Каждая функция сопровождается тестами.

| Задача | Решение |
|---|---|
| 1 | `count_frequencies`: частоты элементов через `HashMap::entry` |
| 2 | `filter_and_uppercase`: `filter`, `map` и `collect` |
| 2.1 | `find_min`: минимум через `Iterator::min`, включая пустой вектор |
| 2.2 | `sum_elements`: сумма через `Iterator::sum` |
| 2.3 | `remove_all_occurrences`: удаление всех вхождений через `Vec::retain` |
| 2.4 | `is_palindrome`: сравнение итератора с обратным итератором |
| 3 | `process_queue`: FIFO через `VecDeque::push_back` и `pop_front` |
| 4 | `unique_elements`: удаление дубликатов через `HashSet` |
| 5 | `sort_by_age_descending`: сортировка `Person` через `sort_by` |
| 6 | `merge_unique`: объединение множеств через `HashSet::union` |
| 7 | `first_unique_element`: частоты и поиск первого элемента с частотой 1 |
| 8 | `group_by_length`: группировка строк через `HashMap<usize, Vec<String>>` |
| 9 | `process_numbers`: чётные числа и умножение на 2 |
| 10 | `find_long_string`: первая строка с `len() > 5` или `"Not found"` |

## Важные детали

- Порядок результатов `unique_elements` и `merge_unique` не гарантирован, поэтому тесты сортируют их перед сравнением.
- В решении задачи 10 на слайде-ссылке комментарий называет результатом `"tiger"`, но его длина равна 5. Согласно условию `len() > 5`, функция возвращает `"elephant"`.
- Длина строк в задачах 2, 8 и 10 вычисляется методом `str::len`, то есть в байтах, как в исходных решениях.
- Внешние зависимости не используются.

## Запуск

```bash
cargo run
```

Запуск тестов:

```bash
cargo test
```
