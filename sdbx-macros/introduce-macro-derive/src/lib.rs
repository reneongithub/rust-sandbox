extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Introducer)]
pub fn introduce_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_introduce_macro(&ast)
}

fn impl_introduce_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Introducer for #name {
            fn introduce(&self){
                println!("I am {}.", stringify!(#name) );
            }
            fn get_type_name(&self) -> String{
                String::from(stringify!(#name))
            }
        }

    };

    gen.into()
}
