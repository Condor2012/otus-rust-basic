pub struct MyStruct {
    value: u32,
}

impl MyStruct {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    let value = MyStruct::new(42);
    println!("Значение из инкапсулированного поля: {}", value.value());
}

#[cfg(test)]
mod tests {
    use super::MyStruct;

    #[test]
    fn method_reads_private_field() {
        assert_eq!(MyStruct::new(42).value(), 42);
    }
}
