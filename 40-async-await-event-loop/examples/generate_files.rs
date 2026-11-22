use std::path::Path;

use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

const TARGET_SIZE: usize = 100 * 1024 * 1024; // 100 Мбайт
const BUFFER_SIZE: usize = 1024 * 1024; // 1 Мбайт

async fn generate_large_file(filename: &Path) -> std::io::Result<()> {
    let file = File::create(filename).await?;
    let mut writer = BufWriter::new(file);
    let pattern = b"0123456789";
    let pattern_len = pattern.len();
    let mut written = 0;

    while written < TARGET_SIZE {
        let to_write = (TARGET_SIZE - written).min(BUFFER_SIZE);
        let mut buffer = Vec::with_capacity(to_write);

        while buffer.len() < to_write {
            let chunk_start = buffer.len() % pattern_len;
            let chunk_end = (chunk_start + (to_write - buffer.len())).min(pattern_len);
            buffer.extend_from_slice(&pattern[chunk_start..chunk_end]);
        }

        writer.write_all(&buffer).await?;
        written += to_write;
    }

    writer.flush().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_files = ["file1.txt", "file2.txt", "file3.txt"];
    let mut join_handles = vec![];

    for filename in test_files {
        let file_path = Path::new(filename);
        if !file_path.exists() {
            join_handles.push(tokio::spawn(generate_large_file(file_path)));
        }
    }

    for handle in join_handles {
        handle.await??;
    }

    println!("Тестовые файлы готовы");
    Ok(())
}
