# Занятие 40. Асинхронность. Event Loop. async/.await

Примеры перенесены из презентации и исходного репозитория [`async_examples_rdb`](https://github.com/greyshaman/async_examples_rdb). Ошибки чтения файлов обрабатываются через `Result` вместо `unwrap`.

## Что демонстрируют примеры

- `tree_files_sync` — последовательное блокирующее чтение трёх файлов через `std::fs`.
- `tree_files_async` — конкурентное чтение через `tokio::fs` и `tokio::join!`.
- `lazy` — ленивость `Future`: тело async-функции не выполняется при создании и отбрасывании future, а запускается при `.await`.
- `async1` — последовательный async-код и упрощённый ручной `StateMachine`, показывающий переходы через `Future::poll`, `Poll::Pending` и `Poll::Ready`.
- `src/lib.rs` — тестируемая демонстрация ленивости и расчёт памяти для модели «один поток на соединение» из задачи C10K.

Event Loop исполнителя Tokio вызывает `poll` у готовых задач. При `Poll::Pending` задача ждёт события, а `Waker` сообщает исполнителю, когда её нужно poll'ить снова. Благодаря этому поток может выполнять другие задачи во время ожидания I/O.

## Запуск

Сначала создайте три тестовых файла по 100 Мбайт:

```bash
cargo run --example generate_files
```

Затем сравните синхронное и асинхронное чтение:

```bash
cargo run --example tree_files_sync
cargo run --example tree_files_async
```

Остальные примеры и тесты:

```bash
cargo run --example lazy
cargo run --example async1
cargo test
```

Сгенерированные `file1.txt`, `file2.txt`, `file3.txt` исключены из Git. Для работы используется Tokio; runtime не входит в стандартную библиотеку Rust.

## Ожидаемый вывод

Пример `lazy` наглядно показывает, что создание и отбрасывание future не запускают её тело — выполнение начинается только после `.await`:

```text
До await started = false
После drop started = false
После await: result = 42, started = true
```

После запуска `generate_files` пример `async1` выводит размеры трёх созданных файлов:

```text
Прочитано: 104857600 байт, 104857600 байт, 104857600 байт
```

В исходнике `async1` также можно проследить переходы ручной `StateMachine` и вызовы `poll`. Для `tree_files_sync` и `tree_files_async` фиксированный вывод не приведён: он зависит от содержимого файлов.
