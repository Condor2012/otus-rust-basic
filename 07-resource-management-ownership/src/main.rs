use std::mem::size_of;

static STATIC_TEXT: &str = "Текст в статически выделенной памяти";

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("Ресурс '{name}' инициализирован");
        Self {
            name: name.to_owned(),
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Ресурс '{}' освобождён через Drop", self.name);
    }
}

fn pass_and_return(mut value: String) -> String {
    value.push_str(" — владение передано в функцию и возвращено");
    value
}

fn copy_number(value: i32) -> i32 {
    value * 2
}

fn main() {
    println!("=== Память ===");
    println!("{STATIC_TEXT}");

    let stack_array = [1_i32, 2, 3, 4, 5];
    let heap_string = String::from("Строка владеет буфером в куче");

    println!("Размер i32: {} байт", size_of::<i32>());
    println!("Размер [i32; 5]: {} байт", size_of::<[i32; 5]>());
    println!("Размер String: {} байт", size_of::<String>());
    println!("Размер &str: {} байт", size_of::<&str>());
    println!("Адрес локального массива: {:p}", &stack_array);
    println!("Адрес структуры String: {:p}", &heap_string);
    println!("Адрес буфера строки в куче: {:p}", heap_string.as_ptr());

    println!("\n=== RAII и область видимости ===");
    {
        let resource = Resource::new("временный ресурс");
        println!("Сейчас можно пользоваться ресурсом '{}'", resource.name);
    }
    println!("Область видимости временного ресурса завершена");

    println!("\n=== Перемещение String ===");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("Новый владелец s2: {s2}");

    let cloned = s2.clone();
    println!("Оригинал после clone: {s2}");
    println!("Независимый клон: {cloned}");
    println!("Адрес буфера оригинала: {:p}", s2.as_ptr());
    println!("Адрес буфера клона: {:p}", cloned.as_ptr());

    let returned = pass_and_return(cloned);
    println!("Возвращённая строка: {returned}");

    println!("\n=== Copy ===");
    let x = 21_i32;
    let y = x;
    println!("x = {x}, y = {y}: обе переменные доступны");
    println!("copy_number(x) = {}", copy_number(x));

    println!("\n=== Перемещение собственного ресурса ===");
    {
        let resource = Resource::new("перемещаемый ресурс");
        let new_owner = resource;
        println!("Новый владелец использует '{}'", new_owner.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_ownership_returns_from_function() {
        let input = String::from("hello");
        let output = pass_and_return(input);

        assert!(output.starts_with("hello"));
        assert!(output.contains("владение передано"));
    }

    #[test]
    fn cloned_strings_are_independent() {
        let original = String::from("hello");
        let mut cloned = original.clone();
        cloned.push_str(" world");

        assert_eq!(original, "hello");
        assert_eq!(cloned, "hello world");
        assert_ne!(original.as_ptr(), cloned.as_ptr());
    }

    #[test]
    fn copy_number_keeps_source_usable() {
        let value = 21;
        let copy = value;

        assert_eq!(value, 21);
        assert_eq!(copy, 21);
        assert_eq!(copy_number(value), 42);
    }
}
