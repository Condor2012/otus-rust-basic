# Otus: Rust Developer. Basic

Примеры и решения практических задач к курсу Otus **«Rust-разработчик. Базовый уровень»**.

Репозиторий организован по занятиям. В каждом кодовом каталоге находятся самостоятельный crate, русскоязычный README, запускаемые примеры и, где это полезно, тесты. Все crate используют Rust edition 2024 и объединены в корневой workspace.

## Быстрый старт

Для большинства занятий достаточно перейти в каталог и запустить основной пример:

```bash
cd 01-introduction
cargo run --example demo
cargo test
```

Занятия 32, 33, 35 и 40 содержат несколько примеров: их имена и команды перечислены в README соответствующего каталога. Занятие 41 содержит ссылку на полноценный проект из курса Professional.

Проверка всего репозитория из корня:

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Пример `deadlock` из занятия 33 намеренно не завершается; он предназначен только для ручной демонстрации взаимной блокировки.

## Программа курса

### Модуль 1. Введение

| Занятие | Тема | Материалы |
|---|---|---|
| 1 | Знакомство с Rust. Подготовка к обучению | [`01-introduction/`](01-introduction/) |
| 2 | Основы архитектуры компьютера | [`02-computer-architecture-basics/`](02-computer-architecture-basics/) |

### Модуль 2. Синтаксис Rust и основы Computer Science

| Занятие | Тема | Материалы |
|---|---|---|
| 3 | Типы данных. Переменные и функции | [`03-data-types-variables-functions/`](03-data-types-variables-functions/) |
| 4 | Сложные типы данных. Кортеж, структура, перечисление | [`04-tuples-structs-enums/`](04-tuples-structs-enums/) |
| 5 | Ветвление и циклы | [`05-branching-and-loops/`](05-branching-and-loops/) |
| 6 | Практика: типы данных, переменные, ветвление, циклы | [`06-practice-data-types-control-flow/`](06-practice-data-types-control-flow/) |
| 7 | Управление ресурсами. Клонирование. Copy. Владение | [`07-resource-management-ownership/`](07-resource-management-ownership/) |
| 8 | Владение. Ссылки и заимствование | [`08-ownership-references-borrowing/`](08-ownership-references-borrowing/) |
| 9 | Практика: владение и заимствование | [`09-ownership-borrowing-practice/`](09-ownership-borrowing-practice/) |
| 10 | Коллекции. Слайсы, массивы, строки | [`10-collections-slices-strings/`](10-collections-slices-strings/) |
| 11 | Q&A-сессия | Без искусственного примера |
| 12 | Структуры и методы. Инкапсуляция | [`12-structs-methods-encapsulation/`](12-structs-methods-encapsulation/) |
| 13 | Перечисления и pattern matching | [`13-enums-pattern-matching/`](13-enums-pattern-matching/) |
| 14 | Практика: структуры, перечисления, pattern matching | [`14-structs-enums-pattern-matching-practice/`](14-structs-enums-pattern-matching-practice/) |
| 15 | Система типов. Result, Option | [`15-type-system-result-option/`](15-type-system-result-option/) |
| 16 | Лямбда-функции | [`16-closures-introduction/`](16-closures-introduction/) |
| 17 | Обработка ошибок. Оператор `?` | [`17-error-handling-question-mark/`](17-error-handling-question-mark/) |
| 18 | Практика: Result, Option | [`18-result-option-practice/`](18-result-option-practice/) |

### Модуль 3. Типы, полиморфизм и дженерики

| Занятие | Тема | Материалы |
|---|---|---|
| 19 | Обобщённое программирование. Статический полиморфизм | [`19-generics-static-polymorphism/`](19-generics-static-polymorphism/) |
| 20 | Q&A-сессия | Без искусственного примера |
| 21 | Времена жизни | [`21-lifetimes/`](21-lifetimes/) |
| 22 | Supertraits, ассоциированные элементы и const generics | [`22-supertraits-associated-items-const-generics/`](22-supertraits-associated-items-const-generics/) |
| 23 | Динамический полиморфизм. `dyn Trait`, `dyn Any` | [`23-dynamic-polymorphism/`](23-dynamic-polymorphism/) |
| 24 | Практика: обобщённое программирование | [`24-generics-lifetimes-practice/`](24-generics-lifetimes-practice/) |
| 25 | Коллекции. Big O. Хэш-карта и вектор | [`25-collections-big-o-vec-hashmap/`](25-collections-big-o-vec-hashmap/) |
| 26 | Коллекции. Итераторы | [`26-collections-iterators/`](26-collections-iterators/) |
| 27 | Практика: коллекции и итераторы | [`27-collections-iterators-practice/`](27-collections-iterators-practice/) |
| 28 | Функции. Типы функций. Лямбда-функции | [`28-function-types-closures/`](28-function-types-closures/) |
| 29 | Q&A-сессия | Без искусственного примера |

### Модуль 4. Многопоточность и дополнительные темы

| Занятие | Тема | Материалы |
|---|---|---|
| 30 | Работа с кучей. Box, Arc, Rc. RAII | [`30-heap-box-rc-arc-raii/`](30-heap-box-rc-arc-raii/) |
| 31 | Потоки. Send + Sync | [`31-threads-send-sync/`](31-threads-send-sync/) |
| 32 | Лямбды и потоки. move и Arc | [`32-closures-threads-arc/`](32-closures-threads-arc/) |
| 33 | Mutex, Atomic | [`33-mutex-atomic/`](33-mutex-atomic/) |
| 34 | Практика: потоки, куча, mutex, atomic | [`34-threads-mutex-atomic-practice/`](34-threads-mutex-atomic-practice/) |
| 35 | Interior mutability. Cell, RefCell | [`35-interior-mutability/`](35-interior-mutability/) |
| 36 | Q&A-сессия | Без искусственного примера |
| 37 | Сборка. Модули | [`37-build-modules/`](37-build-modules/) |
| 38 | Продвинутые элементы синтаксиса | [`38-advanced-syntax/`](38-advanced-syntax/) |
| 39 | Макросы: декларативные и процедурные | [`39-macros/`](39-macros/) |
| 40 | Асинхронность. Event loop. async/await | [`40-async-await-event-loop/`](40-async-await-event-loop/) |
| 41 | Разбор приложения: ThreadPool | [`41-application-review-thread-pool/`](41-application-review-thread-pool/) |
| 42 | Q&A-сессия | Без искусственного примера |

### Модуль 5. Проектная работа

| Занятия | Тема | Материалы |
|---|---|---|
| 43–45 | Выбор темы, консультация и защита проектных работ | Индивидуальные проекты |

## Организация кода

- исполняемые демонстрации находятся в `examples/` и запускаются через `cargo run --example <имя>`;
- повторно используемый код и решения задач находятся в `src/` и покрываются тестами;
- Q&A-занятия 11, 20, 29, 36 и 42 не содержат искусственных примеров;
- внешние зависимости используются только там, где они относятся к теме: `parking_lot` в занятии 33, `syn` и `quote` в занятии 39, Tokio в занятии 40;
- каталоги `target/`, файлы `Cargo.lock` и создаваемые примером занятия 40 большие текстовые файлы не публикуются.
