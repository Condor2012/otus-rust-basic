use lesson39_proc_macros::{Count, identity, keep};

#[allow(dead_code)]
#[derive(Count)]
enum ThreeVariants {
    First,
    Second(i32),
    Third { value: String },
}

#[allow(dead_code)]
#[derive(Count)]
enum Empty {}

#[keep]
struct KeptStruct {
    value: i32,
}

#[keep(some_input)]
fn kept_function() -> i32 {
    7
}

#[test]
fn derive_counts_all_variant_forms() {
    assert_eq!(ThreeVariants::count(), 3);
    assert_eq!(Empty::count(), 0);
}

#[test]
fn attribute_macro_preserves_items() {
    let value = KeptStruct { value: 5 };
    assert_eq!(value.value, 5);
    assert_eq!(kept_function(), 7);
}

#[test]
fn function_like_macro_preserves_expression() {
    assert_eq!(identity!(2 + 2), 4);
}
