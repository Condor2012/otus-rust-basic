use build_modules::catalog::{Catalog, Product};
use build_modules::pricing::discounted_price;

fn main() {
    let mut catalog = Catalog::new();
    catalog.add(Product::new("Клавиатура", 5_000));
    catalog.add(Product::new("Мышь", 2_500));

    println!("Товаров в каталоге: {}", catalog.len());
    println!("Общая стоимость: {}", catalog.total_price());

    if let Some(product) = catalog.find("Клавиатура") {
        let price = discounted_price(product.price(), 10).unwrap();
        println!("{} со скидкой: {price}", product.name());
    }
}
