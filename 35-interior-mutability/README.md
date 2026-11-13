# Занятие 35. Interior mutability. Cell, RefCell

Мутабельность через неизменяемые ссылки: `Cell<T>` и `RefCell<T>`.

## Что демонстрирует

- `Cell::get/set/replace/take` — мутация значения через разделяемую ссылку
- `RefCell::borrow/borrow_mut` — динамическая проверка правил заимствования во время исполнения
- несколько читателей (`borrow`) одновременно

## Запуск

```bash
cargo run
```
