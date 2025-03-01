use std::str::FromStr;

use crate::Compile;
use proc_macro2::{Group, Ident};

use quote::quote;

pub fn compile_token_stream(
    compile: Compile,
    name: Ident,
    from_int: u64,
    to_int: u64,
) -> proc_macro2::TokenStream {
    // output
    let output = (from_int..to_int)
        .map(|x| {
            let new_inner = update_group_ident(compile.inner.clone(), &name, x);
            quote! {#new_inner}
        })
        .collect::<proc_macro2::TokenStream>();

    output.into()
}

fn update_group_ident(
    tokenstream: proc_macro2::TokenStream,
    name: &Ident,
    index: u64,
) -> proc_macro2::TokenStream {
    // for current_token in tokenstream {
    tokenstream
        .into_iter()
        .map(|current_token| {
            // if current_token is Group {
            if let proc_macro2::TokenTree::Group(ref group) = current_token {
                let delimiter = group.delimiter();
                // if current_token'stream length is 1 {
                if group.stream().into_iter().count() == 1 {
                    // if current_token == "N"
                    if group.stream().to_string() == name.to_string() {
                        //  {current_token = customized_token //change current token.}
                        let new_token =
                            proc_macro2::TokenStream::from_str(&index.to_string()).unwrap();
                        // replace group's name with specific number
                        let new_group = Group::new(delimiter, new_token.clone());
                        let customized_token = proc_macro2::TokenTree::Group(new_group);
                        return customized_token;
                    } else {
                        return current_token;
                    }
                } else {
                    let inner_tokenstream = update_group_ident(group.stream(), name, index);
                    let new_group = Group::new(delimiter, inner_tokenstream.clone());
                    let customized_token = proc_macro2::TokenTree::Group(new_group);
                    return customized_token;
                }
            }
            return current_token;
        })
        .collect::<proc_macro2::TokenStream>()
}
