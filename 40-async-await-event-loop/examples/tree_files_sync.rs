use std::{fs, time::Instant};

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    println!("Читаем первый файл...");
    let file1 = std::fs::read_to_string("file1.txt")?;
    println!("Читаем второй файл...");
    let file2 = fs::read_to_string("file2.txt")?;
    println!("Читаем третий файл...");
    let file3 = fs::read_to_string("file3.txt")?;

    println!("Прочитано: {}, {}, {} байт", file1.len(), file2.len(), file3.len());
    println!("Всё прочитано за {:?}", start.elapsed());
    Ok(())
}
