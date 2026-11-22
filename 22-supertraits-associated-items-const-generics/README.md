# Занятие 22. Supertraits, extension traits, ассоциированные элементы и const generics

Примеры построены на PDF и файлах из архива `advtraits`.

## Supertraits и extension traits

- `ToStringExt: Display` расширяет любой тип с `Display` методами форматирования;
- blanket implementation `impl<T: Display> ToStringExt for T`;
- `VecExt<T>` добавляет векторам метод `ends`;
- `IntExt` добавляет `is_even` целым числам.

## Ассоциированные типы и константы

- `BoundedNumeric` задаёт `MIN`, `MAX` и `ZERO`;
- `Celsius` и `Percentage` имеют разные границы, а `normalize` ограничивает значения ими;
- `MaybeFrom<T>` содержит ассоциированный тип `Error`;
- реализация `MaybeFrom<f32>` для `i32` проверяет конечность, диапазон и отсутствие дробной части;
- функция `handle_iterator` показывает ограничение `T::Item`.

## Const generics

- `take_n<T, I, const N: usize>` формирует массив нужной длины, дополняя недостающие элементы через `Default`;
- `Buffer<const N: usize>` хранит массив `[u8; N]`;
- `Buffer<10>` и `Buffer<20>` — разные типы, поэтому значение одного нельзя присвоить переменной другого.

## Запуск

```bash
cargo run
```

Запуск тестов:

```bash
cargo test
```
