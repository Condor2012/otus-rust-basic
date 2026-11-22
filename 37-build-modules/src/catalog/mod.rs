mod item;

pub use item::Product;

#[derive(Default)]
pub struct Catalog {
    products: Vec<Product>,
}

impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn find(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|product| product.name() == name)
    }

    pub fn total_price(&self) -> u64 {
        self.products.iter().map(Product::price).sum()
    }

    pub fn len(&self) -> usize {
        self.products.len()
    }

    pub fn is_empty(&self) -> bool {
        self.products.is_empty()
    }
}
