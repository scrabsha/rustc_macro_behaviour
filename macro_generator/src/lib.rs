use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{parse_macro_input, DeriveInput};

use quote::quote;

// The proc_macro entry point.
#[proc_macro_derive(Cake)]
pub fn cake_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let style = choose_ordering_style(&input);

    style.expand_to_code().into()
}

// Represents whether if the trait should be placed before or after the struct
// declaration.
#[derive(Copy, Clone, Debug, PartialEq)]
enum OrderingStyle {
    // Struct declaration followed by trait implementation
    StructBeforeTrait,
    // Trait implementation followed by struct declaration
    TraitBeforeStruct,
}

impl OrderingStyle {
    // Generates a fake struct. Its name depends on the ordering style. The
    // ordering "struct declaration - Cake trait implementation" is defined
    // by the current variant.
    fn expand_to_code(self) -> TokenStream2 {
        match self {
            OrderingStyle::StructBeforeTrait => quote! {
                struct NewSchoolCake {};
                impl exporter::Cake for NewSchoolCake {}
            },
            OrderingStyle::TraitBeforeStruct => quote! {
                impl exporter::Cake for OldSchoolCake {}
                struct OldSchoolCake {};
            }
        }
    }
}

// Chooses which ordering style should be used, based on the name of the struct
// on which `#[derive(Cake)]` is applied.
//
// If the first letter of the struct is a vowel, then the `StructBeforeTrait`
// variant is returned. Otherwise, the `TraitBeforeStruct` is returned.
//
// The goal is to show that the ordering matter, so the cake_derive function is
// likely to be called twice, first with a struct name starting with a vowel,
// then starting with a consonant.
fn choose_ordering_style(input: &DeriveInput) -> OrderingStyle {
    // We assume the letter is lowercase, as structs name should be CamelCase.
    match input.ident.to_string().chars().next().unwrap() {
        'A' | 'E'| 'I' | 'O' | 'U'| 'Y' => OrderingStyle::StructBeforeTrait,
        _ => OrderingStyle::TraitBeforeStruct,
    }
}

