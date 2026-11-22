use lesson_23_dynamic_polymorphism::any_examples::{demo_values, describe, same_type};
use lesson_23_dynamic_polymorphism::drawable::{Circle, Rect};
use lesson_23_dynamic_polymorphism::dynamic_dispatch::{DynCanvas, describe_borrowed};
use lesson_23_dynamic_polymorphism::enum_dispatch::EnumCanvas;

fn main() {
    let mut dynamic_canvas = DynCanvas::default();
    dynamic_canvas.add(Rect::new(0, 0, 20, 10));
    dynamic_canvas.add(Rect::new(10, 10, 15, 10));
    dynamic_canvas.add(Circle::new(15, 5, 5));
    dynamic_canvas.add(Circle::new(25, 15, 5));

    println!("=== Динамическая диспетчеризация: Box<dyn Drawable> ===");
    dynamic_canvas.draw();

    let circle = Circle::new(1, 2, 3);
    println!("\n=== Заимствованный trait-объект ===");
    println!("{}", describe_borrowed(&circle));

    let mut enum_canvas = EnumCanvas::default();
    enum_canvas.add(Rect::new(0, 0, 20, 10));
    enum_canvas.add(Circle::new(15, 5, 5));

    println!("\n=== Диспетчеризация через перечисление ===");
    enum_canvas.draw();

    println!("\n=== dyn Any и downcast ===");
    for value in demo_values() {
        println!("{}", describe(value.as_ref()));
    }
    println!("u32 и u32 — один тип: {}", same_type::<u32, u32>());
    println!("u32 и String — один тип: {}", same_type::<u32, String>());
}
