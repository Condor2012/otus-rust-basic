// Занятие 4. Сложные типы данных. Кортеж, структура, перечисление

use std::mem::{align_of, size_of};

// ---- Структуры с именованными полями ----
// Имя типа — PascalCase, имена полей — snake_case.
// pub — поле публично, pub(crate) — видно только внутри текущего крейта,
// без модификатора — приватное поле.
#[derive(Debug, Default)]
struct Foo {
    pub a: u32,
    pub(crate) b: u8,
}

// ---- Структура-кортеж ----
#[derive(Debug)]
struct Bar(u32, bool);

// ---- Юнит-структура ----
struct Baz;

// ---- Перечисление ----
// Тип-сумма (тегированный юнион). Варианты могут быть юнитами,
// кортежами и структурами с именованными полями.
#[derive(Debug)]
enum Shape {
    Empty,
    Pair(u32, u64),
    Rect { width: u32, height: u32 },
}

// ---- Явные теги перечисления ----
// #[repr(u32)] задаёт тип тега и позволяет кастовать вариант в число.
#[repr(u32)]
#[derive(Debug)]
enum Tagged {
    A = 42,
    B = 24,
    C = 134,
}

// ---- Type layout ----
// Компилятор может менять порядок полей и вставлять паддинг.
// #[repr(C)] фиксирует порядок полей как в C.
#[repr(C)]
struct ReprC {
    a: u8,
    b: u32,
}

// #[repr(align(8))] повышает выравнивание типа.
#[repr(align(8))]
struct Aligned(u8);

fn main() {
    // ---- Кортежи ----
    // Типы элементов могут быть одинаковыми или разными, допускается вложенность.
    let t: (u32, u8, u64) = (42, 13, 1000);
    println!("t.0 = {}, t.1 = {}, t.2 = {}", t.0, t.1, t.2);

    let nested: (u32, (u64, u8), u32) = (1, (2, 3), 4);
    println!("nested.1.0 = {}", nested.1.0);

    // Компилятор автоматически выводит некоторые типажи для кортежа:
    println!("t = {t:?}");

    // Пустой кортеж () — единичный тип (unit).
    // Функции, которые ничего не возвращают, на самом деле возвращают ().
    let unit: () = { println!("этот блок вычисляется в ()"); };
    println!("unit = {unit:?}");

    // ---- Структуры с именованными полями ----
    let x = Foo { a: 42, b: 13 };
    println!("x.a = {}, x.b = {}", x.a, x.b);

    // Сокращённая инициализация: имена переменных совпадают с полями
    let a = 50;
    let b = 34;
    let x = Foo { a, b };

    // Синтаксис ..: оставшиеся поля копируются из другой структуры
    let y = Foo { a: 50, ..x };
    println!("y = {y:?}");

    // Частая вариация: остальные поля — по умолчанию
    let z = Foo { a: 50, ..Default::default() };
    println!("z = {z:?}");

    // ---- Структуры-кортежи ----
    let bar = Bar(42, true);
    println!("bar.0 = {}, bar.1 = {}", bar.0, bar.1);

    // Тип структуры-кортежа неявно является функцией-конструктором:
    let ctor: fn(u32, bool) -> Bar = Bar;
    println!("ctor(1, false) = {:?}", ctor(1, false));

    // ---- Юнит-структуры ----
    let _baz = Baz; // без скобок, в отличие от Bar()
    println!("юнит-структура Baz создана");

    // ---- Перечисления и match ----
    // Доступ к данным варианта требует проверки типа — сопоставления паттернов.
    let shapes = [
        Shape::Empty,
        Shape::Pair(1, 2),
        Shape::Rect { width: 3, height: 4 },
    ];
    for shape in &shapes {
        match shape {
            Shape::Empty => println!("пустой вариант"),
            Shape::Pair(a, b) => println!("пара: {a}, {b}"),
            Shape::Rect { width, height } => {
                println!("прямоугольник {width}x{height}, площадь {}", width * height)
            }
        }
    }

    // ---- Явные теги перечисления ----
    println!("Tagged::A as u32 = {}", Tagged::A as u32);
    println!("Tagged::C as u32 = {}", Tagged::C as u32);

    // ---- Type layout ----
    println!("size_of::<u8>() = {}", size_of::<u8>());
    println!("size_of::<Foo>() = {}", size_of::<Foo>());
    println!("size_of::<ReprC>() = {} (с паддингом)", size_of::<ReprC>());
    println!("align_of::<ReprC>() = {}", align_of::<ReprC>());
    println!("size_of::<Aligned>() = {}", size_of::<Aligned>());
    println!("align_of::<Aligned>() = {}", align_of::<Aligned>());
    println!("size_of::<Shape>() = {}", size_of::<Shape>());
}
