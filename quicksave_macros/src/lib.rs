extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn save_slot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let strukt: ItemStruct = parse(item).unwrap();

    let fields = if let Fields::Named(fields) = &strukt.fields {
        fields
    } else {
        panic!("Only structs with named fields are supported");
    };

    let name = strukt.ident;
    let gen = quote! {
        struct #name
            #fields
    };

    gen.into()
}
