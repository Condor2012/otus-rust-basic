use lesson_22_supertraits_associated_items_const_generics::buffer::Buffer;
use lesson_22_supertraits_associated_items_const_generics::maybe_from::MaybeFrom;
use lesson_22_supertraits_associated_items_const_generics::normalize::{
    BoundedNumeric, Celsius, Percentage, normalize,
};
use lesson_22_supertraits_associated_items_const_generics::str_ext::{IntExt, ToStringExt, VecExt};
use lesson_22_supertraits_associated_items_const_generics::take::take_n;

fn main() {
    println!("ToStringExt: {}", 123.to_square_str());
    println!("VecExt: {:?}", vec![1, 2, 3].ends());
    println!("42 чётное: {}", 42.is_even());

    let temperature = normalize(Celsius(-300.0));
    let percentage = normalize(Percentage(150.0));
    println!("Нормализованная температура: {:?}", temperature);
    println!("Нормализованный процент: {:?}", percentage);
    println!("Celsius::MIN: {:?}", Celsius::MIN);

    println!("42.0 -> i32: {:?}", i32::maybe_from(42.0));
    println!("42.5 -> i32: {:?}", i32::maybe_from(42.5));

    let array: [u32; 7] = take_n(vec![1, 2, 3]);
    println!("take_n::<7>: {array:?}");

    let small = Buffer::<10>::default();
    let large = Buffer::<20>::default();
    println!("Buffer<10>: {} байт", small.len());
    println!("Buffer<20>: {} байт", large.len());
}
