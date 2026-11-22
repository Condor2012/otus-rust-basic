// Занятие 5. Ветвление и циклы

#[derive(Debug)]
enum Key {
    Enter,
    Space,
    Other(char),
}

#[derive(Debug)]
enum Event {
    Key(Key),
    Quit,
}

fn main() {
    // ---- Блок кода как выражение ----
    // Блок содержит инструкции, а последнее выражение без `;` — его результат.
    let sum = {
        let x = 1;
        let y = 2;
        x + y
    };
    println!("сумма из блока = {sum}");

    // ---- if / else if / else ----
    // Условие не оборачивается в скобки, его тип должен быть bool.
    let a = 5;
    let b = 10;
    if a > 0 && b > 0 {
        println!("оба положительные");
    }
    if a == b {
        println!("равны");
    } else if a > b {
        println!("a больше");
    } else {
        println!("b больше");
    }

    // ---- if как выражение ----
    let x = 150;
    let y = if x > 100 { 2025 } else { 2026 };
    println!("y = {y}");
    // Результат можно использовать внутри другого выражения:
    let z = 0 + if x > 100 { 2025 } else { 2026 };
    println!("z = {z}");

    // Отдельностоящий if: `;` в конце не нужен, тип блоков — ()
    if x > 100 {
        println!("x больше 100");
    } else {
        println!("x не больше 100");
    }

    // ---- if let ----
    let opt: Option<u32> = Some(200);
    if let Some(value) = opt {
        println!("есть значение: {value}");
    } else {
        println!("значения нет");
    }

    let res: Result<u32, &str> = Ok(7);
    if let Ok(value) = res {
        println!("Ok: {value}");
    }

    // if let как выражение
    let v = if let Some(value) = opt {
        value
    } else if a < 100 {
        a
    } else {
        0
    };
    println!("v = {v}");

    // if-let-цепочки (доступно в Rust 2024)
    if let Some(value) = opt && value > 100 {
        println!("значение {value} больше 100");
    }
    if let Some(x) = opt && let Some(y) = Some(150_u32) && x > y {
        println!("{x} > {y}");
    }

    // ---- while ----
    let mut n = 0;
    while n < 3 {
        println!("while: n = {n}");
        n += 1;
    }

    // break и continue
    let mut n = 0;
    while n < 10 {
        n += 1;
        if n == 2 {
            continue; // пропускаем итерацию
        }
        if n == 4 {
            break; // выходим из цикла
        }
        println!("n = {n}");
    }

    // ---- while let ----
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("while let: pop -> {top}");
    }

    // ---- loop ----
    // Бесконечный цикл с выходом по break
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts >= 3 {
            break;
        }
    }
    println!("loop: попыток = {attempts}");

    // loop как выражение: break возвращает значение
    let mut ticks = 0;
    let result = loop {
        ticks += 1;
        if ticks == 5 {
            break ticks * 10;
        }
    };
    println!("результат loop = {result}");

    // ---- for и диапазоны ----
    // 0..3 — это std::ops::Range { start: 0, end: 3 };
    // нижняя граница включена, верхняя — нет
    for i in 0..3 {
        println!("for: i = {i}");
    }

    let mut arr = [1, 2, 3];
    for item in &arr {
        println!("по ссылке: {item}");
    }
    for item in &mut arr {
        *item *= 10;
    }
    println!("после iter_mut: {arr:?}");
    for item in arr {
        println!("по значению: {item}");
    }

    // enumerate: индекс + значение, шаблон в for
    let bytes: [u8; 3] = [10, 20, 30];
    for (idx, item) in bytes.into_iter().enumerate() {
        println!("{idx}: {item}");
    }

    // ---- Метки циклов и блоков ----
    // break 'outer выходит сразу из внешнего цикла
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 4 {
                break 'outer;
            }
            println!("i = {i}, j = {j}");
        }
    }

    // Метка блока: break 'tag со значением
    let value = 'tag: {
        let x = -5;
        if x < 0 {
            break 'tag 0;
        }
        x
    };
    println!("значение из блока с меткой = {value}");

    // ---- Эмуляция do-while ----
    // Тело выполняется минимум один раз
    let mut counter = 0;
    loop {
        counter += 1;
        println!("do-while: counter = {counter}");
        if counter == 2 {
            break;
        }
    }

    // ---- match ----
    let events = [Event::Key(Key::Enter), Event::Key(Key::Space), Event::Key(Key::Other('a')), Event::Quit];
    for event in &events {
        match event {
            Event::Key(Key::Enter) => println!("нажат Enter"),
            Event::Key(Key::Space) => println!("нажат пробел"),
            Event::Key(key) => println!("другая клавиша: {key:?}"),
            Event::Quit => println!("выход"),
        }
    }
    // Порядок шаблонов имеет значение: более общие — после частных.
    // `_` — ветка «все остальные».
    let number = 7;
    match number {
        0 => println!("ноль"),
        1..=9 => println!("одна цифра"),
        _ => println!("что-то другое"),
    }

    // ---- Оператор ? ----
    println!("describe(Some(true)) = {:?}", describe(Some(true)));
    println!("describe(None) = {:?}", describe(None));

    // ---- let-else ----
    // Если шаблон не совпал — блок else обязан завершиться
    // return/break/continue/panic!.
    let some_option: Option<u32> = Some(99);
    let Some(value) = some_option else {
        panic!("ожидалось Some");
    };
    println!("let-else: value = {value}");
}

/// `?` на Option: эквивалент match с ранним return None.
fn describe(flag: Option<bool>) -> Option<String> {
    let value = flag?;
    // то же самое без ?:
    // let value = match flag {
    //     Some(v) => v,
    //     None => return None,
    // };
    Some(format!("флаг = {value}"))
}
