extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse, Fields, FieldsNamed, ItemStruct};

#[proc_macro_attribute]
pub fn save_slot(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let strukt: ItemStruct = parse(item).unwrap();

    let fields = if let Fields::Named(fields) = &strukt.fields {
        fields
    } else {
        panic!("Only structs with named fields are supported");
    };

    let name = strukt.ident;
    let generated_struct = quote! {
        struct #name
            #fields
    };

    let args = named_fields_to_function_args(fields);
    let names = named_fields_to_names(fields);

    let impl_block = quote! {
        impl #name {
            fn new(#args) -> Self {
                Self {
                    #names
                }
            }

            fn save(&self) {

            }
        }
    };

    let output = quote! {
        #generated_struct
        #impl_block
    };
    output.into()
}

fn named_fields_to_function_args(fields: &FieldsNamed) -> TokenStream {
    let args = fields.named.iter().map(|field| {
        let name = &field
            .ident
            .clone()
            .expect("Only named fields are supported");
        let ty = &field.ty;
        quote! { #name: #ty }
    });

    quote! { #(#args),* }
}

fn named_fields_to_names(fields: &FieldsNamed) -> TokenStream {
    let names = fields.named.iter().map(|field| {
        let name = &field
            .ident
            .clone()
            .expect("Only named fields are supported");
        quote! { #name }
    });

    quote! { #(#names),* }
}
