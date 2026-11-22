use std::time::Instant;
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let (file1, file2, file3) = tokio::join!(
        fs::read_to_string("file1.txt"),
        fs::read_to_string("file2.txt"),
        fs::read_to_string("file3.txt"),
    );

    let (file1, file2, file3) = (file1?, file2?, file3?);
    println!(
        "Прочитано: {}, {}, {} байт",
        file1.len(),
        file2.len(),
        file3.len()
    );
    println!("Всё прочитано за {:?}", start.elapsed());
    Ok(())
}
