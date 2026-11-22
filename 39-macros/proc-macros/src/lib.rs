use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Function-like макрос: возвращает полученный поток токенов без изменений.
#[proc_macro]
pub fn identity(input: TokenStream) -> TokenStream {
    input
}

/// Attribute-like макрос: сохраняет аннотированный элемент без изменений.
#[proc_macro_attribute]
pub fn keep(_attributes: TokenStream, annotated_item: TokenStream) -> TokenStream {
    annotated_item
}

/// Генерирует метод count() для enum по числу его вариантов.
///
/// Основа — практический пример Count из репозитория со слайда 73.
#[proc_macro_derive(Count)]
pub fn derive_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variant_count = match &input.data {
        Data::Enum(data) => data.variants.len(),
        _ => {
            return syn::Error::new_spanned(name, "Count можно вывести только для enum")
                .to_compile_error()
                .into();
        }
    };

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #type_generics #where_clause {
            pub fn count() -> usize {
                #variant_count
            }
        }
    }
    .into()
}
