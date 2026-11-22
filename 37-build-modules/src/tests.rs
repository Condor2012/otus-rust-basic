use crate::catalog::{Catalog, Product};
use crate::pricing::discounted_price;

#[test]
fn catalog_stores_and_finds_products() {
    let mut catalog = Catalog::new();
    assert!(catalog.is_empty());

    catalog.add(Product::new("Клавиатура", 5_000));
    catalog.add(Product::new("Мышь", 2_500));

    assert_eq!(catalog.len(), 2);
    assert_eq!(catalog.find("Мышь").map(Product::price), Some(2_500));
    assert_eq!(catalog.total_price(), 7_500);
}

#[test]
fn calculates_discount() {
    assert_eq!(discounted_price(1_000, 15), Some(850));
    assert_eq!(discounted_price(1_000, 0), Some(1_000));
    assert_eq!(discounted_price(1_000, 100), Some(0));
    assert_eq!(discounted_price(1_000, 101), None);
}

#[test]
fn modules_work_together() {
    let product = Product::new("Монитор", 20_000);
    assert_eq!(discounted_price(product.price(), 10), Some(18_000));
}
