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
    // Generates a struct definition based on the ordering style.
    fn gen_struct(self) -> TokenStream2 {
        match self {
            OrderingStyle::StructBeforeTrait => quote! {
                struct NewSchoolCake {};
            },
            OrderingStyle::TraitBeforeStruct => quote! {
                struct OldSchoolCake {};
            }
        }
    }

    // Generates an implementation of the trait Cake, for the corresponding
    // struct.
    //
    // Note that the `say_my_name` function is redefined, so that we can
    // differenciate cases in which the trait is not expanded correctly.
    fn gen_trait(self) -> TokenStream2 {
        match self {
            OrderingStyle::StructBeforeTrait => quote! {
                impl exporter::Cake for NewSchoolCake {}
            },
            OrderingStyle::TraitBeforeStruct => quote! {
                impl exporter::Cake for OldSchoolCake {}
            }
        }
    }

    // Concatenates the output of gen_struct and gen_trait correctly, depending
    // on the ordering style.
    fn expand_to_code(self) -> TokenStream2 {
        let struct_definition = self.gen_struct();
        let trait_implementation = self.gen_trait();
        match self {
            OrderingStyle::StructBeforeTrait => quote! {
                #struct_definition
                #trait_implementation
            },
            OrderingStyle::TraitBeforeStruct => quote! {
                #trait_implementation
                #struct_definition
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

