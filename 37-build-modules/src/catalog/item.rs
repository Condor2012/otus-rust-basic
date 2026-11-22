pub struct Product {
    name: String,
    price: u64,
}

impl Product {
    pub fn new(name: impl Into<String>, price: u64) -> Self {
        Self {
            name: name.into(),
            price,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> u64 {
        self.price
    }
}
