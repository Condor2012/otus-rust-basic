/// Возвращает цену со скидкой в процентах, округляя до целого.
pub fn discounted_price(price: u64, discount_percent: u8) -> Option<u64> {
    if discount_percent > 100 {
        return None;
    }

    Some(price * u64::from(100 - discount_percent) / 100)
}
