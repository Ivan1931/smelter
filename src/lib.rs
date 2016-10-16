#![feature(proc_macro, proc_macro_lib)]
#![cfg(not(test))]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
mod code_gen;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_macro_input(&source).unwrap();

    let expanded = code_gen::expand_builder(ast);

    expanded.to_string().parse().unwrap()
}

