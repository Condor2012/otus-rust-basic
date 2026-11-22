#![allow(dead_code)]

use std::{pin::Pin, task::Poll};
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let content1 = fs::read_to_string("file1.txt").await?;
    let content2 = fs::read_to_string("file2.txt").await?;
    let content3 = fs::read_to_string("file3.txt").await?;

    println!(
        "Прочитано: {} байт, {} байт, {} байт",
        content1.len(), content2.len(), content3.len()
    );
    Ok(())
}

/// Асинхронная функция без сахара async fn.
fn another() -> impl Future<Output = std::io::Result<()>> {
    async {
        let content1 = fs::read_to_string("file1.txt").await?;
        let content2 = fs::read_to_string("file2.txt").await?;
        let content3 = fs::read_to_string("file3.txt").await?;
        println!(
            "Прочитано: {} байт, {} байт, {} байт",
            content1.len(), content2.len(), content3.len()
        );
        Ok(())
    }
}

/// Упрощённая иллюстрация конечного автомата, который строит компилятор.
fn state_machine() -> impl Future<Output = ()> {
    async {
        StateMachine::Start.await;
    }
}

enum StateMachine {
    Start,
    ReadingFile1(Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>>),
    ReadingFile2 {
        content1: String,
        future: Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>>,
    },
    ReadingFile3 {
        content1: String,
        content2: String,
        future: Pin<Box<dyn Future<Output = Result<String, std::io::Error>>>>,
    },
    Printing {
        content1: String,
        content2: String,
        content3: String,
    },
    Done,
}

impl Future for StateMachine {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                StateMachine::Start => {
                    let future = Box::pin(fs::read_to_string("file1.txt"));
                    *self = StateMachine::ReadingFile1(future);
                }
                StateMachine::ReadingFile1(future) => match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(content1)) => {
                        let future = Box::pin(fs::read_to_string("file2.txt"));
                        *self = StateMachine::ReadingFile2 { content1, future };
                    }
                    Poll::Ready(Err(_)) => panic!("Ошибка чтения file1.txt"),
                    Poll::Pending => return Poll::Pending,
                },
                StateMachine::ReadingFile2 { content1, future } => {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(Ok(content2)) => {
                            let content1 = std::mem::take(content1);
                            let future = Box::pin(fs::read_to_string("file3.txt"));
                            *self = StateMachine::ReadingFile3 { content1, content2, future };
                        }
                        Poll::Ready(Err(_)) => panic!("Ошибка чтения file2.txt"),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                StateMachine::ReadingFile3 { content1, content2, future } => {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(Ok(content3)) => {
                            let content1 = std::mem::take(content1);
                            let content2 = std::mem::take(content2);
                            *self = StateMachine::Printing { content1, content2, content3 };
                        }
                        Poll::Ready(Err(_)) => panic!("Ошибка чтения file3.txt"),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                StateMachine::Printing { content1, content2, content3 } => {
                    println!(
                        "Прочитано: {} байт, {} байт, {} байт",
                        content1.len(), content2.len(), content3.len()
                    );
                    *self = StateMachine::Done;
                }
                StateMachine::Done => return Poll::Ready(()),
            }
        }
    }
}
