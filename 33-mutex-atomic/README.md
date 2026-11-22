# Занятие 33. Mutex и Atomic

Шесть независимых примеров демонстрируют синхронизацию потоков, атомарные операции и типичные проблемы блокировок.

## Примеры

- `mutex` — `Mutex`, отличие `try_lock` от `lock` и разделённый счётчик;
- `atomic` — `AtomicUsize`, операции `store`, `try_update`, `load` и `Ordering`;
- `rwlock` — конкурентные читатели и эксклюзивный писатель через `RwLock`;
- `parking_lot` — сравнение размера и выравнивания `std::Mutex` и `parking_lot::Mutex`;
- `poisoned` — отравление мьютекса паникой, `is_poisoned` и `clear_poison`;
- `deadlock` — взаимная блокировка при захвате двух мьютексов в разном порядке.

## Запуск

```bash
cargo run --example mutex
cargo run --example atomic
cargo run --example rwlock
cargo run --example parking_lot
cargo run --example poisoned
cargo run --example deadlock
```

> **Внимание:** `deadlock` намеренно зависает. Запускайте его только для демонстрации взаимной блокировки и прерывайте через Ctrl+C.

Все остальные примеры завершаются самостоятельно. Для сравнения реализаций мьютексов используется crate `parking_lot`.

## Ожидаемый вывод

`mutex` показывает итоговое значение разделённого счётчика:

```text
Result: Counter(20)
```

`atomic` выводит результат атомарных операций:

```text
Result: 100010
```

В конце `poisoned` видно, как `clear_poison` снимает признак отравления:

```text
Is poisoned: true
Poisoned: PoisonError { .. }
Is poisoned: false
Lock: Ok(0)
```

Перед этими строками пример намеренно вызывает панику в отдельном потоке, поэтому Rust также выводит сообщение о ней. Порядок строк `rwlock` зависит от планировщика потоков, а размеры и выравнивание в `parking_lot` — от платформы.
