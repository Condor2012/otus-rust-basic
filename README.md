# otus-rust-basic

Примеры кода к курсу Otus **«Rust-разработчик. Базовый уровень»**.

Каждая папка — материалы одного занятия: рабочий пример и краткое описание.

## Программа курса

### Модуль 1. Введение

| Занятие | Тема | Код |
|---|---|---|
| 1 | Знакомство с Rust. Подготовка к обучению | [`01-introduction/`](01-introduction/) |
| 2 | Основы архитектуры компьютера | [`02-computer-architecture-basics/`](02-computer-architecture-basics/) |

### Модуль 2. Синтаксис Rust и основы Computer Science

| Занятие | Тема | Код |
|---|---|---|
| 3 | Типы данных. Переменные и функции | [`03-data-types-variables-functions/`](03-data-types-variables-functions/) |
| 4 | Сложные типы данных. Кортеж, структура, перечисление | [`04-tuples-structs-enums/`](04-tuples-structs-enums/) |
| 5 | Ветвление и циклы | [`05-branching-and-loops/`](05-branching-and-loops/) |
| 6 | Практика: типы данных, переменные, ветвление, циклы | [`06-practice-data-types-control-flow/`](06-practice-data-types-control-flow/) |
| 7 | Управление ресурсами. Клонирование. Copy. Владение | [`07-resource-management-ownership/`](07-resource-management-ownership/) |
| 8 | Владение. Ссылки и заимствование | [`08-ownership-references-borrowing/`](08-ownership-references-borrowing/) |
| 9 | Практика: владение и заимствование | [`09-ownership-borrowing-practice/`](09-ownership-borrowing-practice/) |
| 10 | Коллекции. Слайсы, массивы, строки | [`10-collections-slices-strings/`](10-collections-slices-strings/) |
| 11 | Q&A сессия | — |
| 12 | Структуры и методы. Инкапсуляция | [`12-structs-methods-encapsulation/`](12-structs-methods-encapsulation/) |
| 13 | Перечисления и pattern matching | [`13-enums-pattern-matching/`](13-enums-pattern-matching/) |
| 14 | Практика: структуры, перечисления, pattern matching | [`14-structs-enums-pattern-matching-practice/`](14-structs-enums-pattern-matching-practice/) |
| 15 | Система типов. Result, Option | [`15-type-system-result-option/`](15-type-system-result-option/) |
| 16 | Лямбда-функции | [`16-closures-introduction/`](16-closures-introduction/) |
| 17 | Обработка ошибок. Оператор ? | [`17-error-handling-question-mark/`](17-error-handling-question-mark/) |
| 18 | Практика: Result, Option | [`18-result-option-practice/`](18-result-option-practice/) |

### Модуль 3. Типы, полиморфизм и дженерики

| Занятие | Тема | Код |
|---|---|---|
| 19 | Обобщённое программирование. Статический полиморфизм | [`19-generics-static-polymorphism/`](19-generics-static-polymorphism/) |
| 20 | Q&A сессия | — |
| 21 | Времена жизни | — |
| 22 | Наследование трейтов. Ассоциированные трейты | — |
| 23 | Динамический полиморфизм. dyn Trait, dyn Any | — |
| 24 | Практика: обобщённое программирование | — |
| 25 | Коллекции. Big O. Хэш-мап и вектор | — |
| 26 | Коллекции. Итераторы | — |
| 27 | Практика: коллекции и итераторы | — |
| 28 | Функции. Типы функций. Лямбда-функции | — |
| 29 | Q&A сессия | — |

### Модуль 4. Многопоточность и дополнительные темы

| Занятие | Тема | Код |
|---|---|---|
| 30 | Работа с кучей. Box, Arc, Rc. RAII | — |
| 31 | Потоки. Send + Sync | — |
| 32 | Лямбды и потоки. move и Arc | [`32-closures-threads-arc/`](32-closures-threads-arc/) |
| 33 | Mutex, Atomic | [`33-mutex-atomic/`](33-mutex-atomic/) |
| 34 | Практика: потоки, куча, mutex, atomic | — |
| 35 | Interior mutability. Cell, RefCell | [`35-interior-mutability/`](35-interior-mutability/) |
| 36 | Q&A сессия | — |
| 37 | Сборка. Модули | — |
| 38 | Продвинутые элементы синтаксиса | — |
| 39 | Макросы: декларативные и процедурные | — |
| 40 | Асинхронность. async/await | — |
| 41 | Пример: разбор приложения на Rust | — |
| 42 | Q&A сессия | — |

### Модуль 5. Проектная работа

| Занятие | Тема | Код |
|---|---|---|
| 43–45 | Выбор темы, консультация, защита проектных работ | — |

## Как запускать

Каждый пример — отдельный crate:

```bash
cd <папка-занятия>
cargo run
```

или, если это example/bin внутри crate:

```bash
cargo run --example <имя>
cargo run --bin <имя>
```
