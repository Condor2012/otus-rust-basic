# Занятие 33. Mutex, Atomic

Песочница для синхронизации потоков: один crate, шесть бинарников.

## Примеры

```bash
cargo run --bin mutex       # Mutex: try_lock vs lock, разделённый счётчик
cargo run --bin atomic      # AtomicUsize: store / fetch_update / load, Ordering
cargo run --bin rwlock      # RwLock: конкурентные читатели vs писатели
cargo run --bin parking_lot # размер и выравнивание std::Mutex vs parking_lot::Mutex
cargo run --bin poisoned    # отравление мьютекса паникой, is_poisoned, clear_poison
cargo run --bin deadlock    # классический дедлок: два мьютекса в разном порядке
```

> ⚠️ `deadlock` намеренно зависает — запускайте, чтобы увидеть взаимную блокировку вживую, и прерывайте по Ctrl+C.
