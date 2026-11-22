# Занятие 41. Пример: разбор приложения на Rust

Для разбора используется готовый пример из курса Professional: [**«Пишем ThreadPool»**](https://github.com/Condor2012/otus-rust-advanced/tree/main/16-thread-pool).

Код не дублируется в этом репозитории. По ссылке доступны реализация пула потоков на `std::sync::mpsc`, корректное завершение через `Drop`, минимальный actor-фреймворк и запускаемые примеры.

## План разбора

1. Публичный API `ThreadPool`: создание пула и отправка задач.
2. Канал `mpsc`, разделение `Receiver` между worker-потоками через `Mutex`.
3. Жизненный цикл задачи и worker-потока.
4. Освобождение ресурсов и ожидание потоков в `Drop`.
5. Примеры `ping_pong`, `barrier`, `send_sync`, `weak_strong`, `phantom_data` и `find`.
6. Сравнение собственного пула с параллельным поиском через Rayon.

Запуск исходного приложения:

```bash
git clone https://github.com/Condor2012/otus-rust-advanced.git
cd otus-rust-advanced/16-thread-pool
cargo run --example find
cargo test
```
