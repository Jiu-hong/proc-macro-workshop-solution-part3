use std::str::FromStr;

use crate::Compile;

use proc_macro2::{Delimiter, Group, Ident, Span};
use quote::{ToTokens, quote};

pub fn compile_token_stream(
    compile: Compile,
    name: Ident,
    from_int: u64,
    to_int: u64,
) -> proc_macro2::TokenStream {
    eprintln!("compile is {:#?}", compile);
    // compile_error!(concat!("error number ", stringify!(N)));
    // let compile_error_name = compile.compile_error_name;
    // let exclamation_mark = compile.exclamation_mark;
    // let inner_ident = compile.inner_ident;
    // let inner_exclamation_mark = compile.inner_exclamation_mark;
    // let error_number = compile.inner_literal;
    // let inner_comma = compile.inner_comma;
    // let stringify = compile.inner_inner_ident;
    // let inner_inner_exclamation_mark = compile.inner_inner_exclamation_mark;
    // // let n_ident = compile.n_ident;
    // let last_semicolon = compile.last_semicolon;

    // let output = (from_int..to_int)
    //     .map(|index| {
    //         let number_ident_token: proc_macro2::TokenStream = index.to_token_stream();

    //         eprintln!("number_ident_token is {:#?}", number_ident_token);

    //         // (N)
    //         let inner_group_3 = Group::new(Delimiter::Parenthesis, number_ident_token);
    //         // compile_error!(concat!("error number ", stringify!(N)));
    //         //
    //         // stringify!(N)
    //         let token_stringify = proc_macro2::TokenStream::from_iter(
    //             [
    //                 stringify.to_token_stream(),
    //                 inner_inner_exclamation_mark.to_token_stream(),
    //                 inner_group_3.to_token_stream(),
    //             ]
    //             .into_iter(),
    //         );
    //         // ("error number ", stringify!(N))
    //         let group2 = Group::new(
    //             Delimiter::Parenthesis,
    //             proc_macro2::TokenStream::from_iter(
    //                 [
    //                     error_number.to_token_stream(),
    //                     inner_comma.to_token_stream(),
    //                     token_stringify,
    //                 ]
    //                 .into_iter(),
    //             ),
    //         );
    //         // concat!("error number ", stringify!(N))
    //         let contact_token = proc_macro2::TokenStream::from_iter(
    //             [
    //                 inner_ident.to_token_stream(),
    //                 inner_exclamation_mark.to_token_stream(),
    //                 group2.to_token_stream(),
    //             ]
    //             .into_iter(),
    //         );

    //         // (concat!("error number ", stringify!(N)))
    //         let group1 = Group::new(
    //             Delimiter::Parenthesis,
    //             proc_macro2::TokenStream::from(contact_token),
    //         );

    //         // compile_error!(concat!("error number ", stringify!(N)));
    //         let output_each = proc_macro2::TokenStream::from_iter(
    //             [
    //                 compile_error_name.to_token_stream(),
    //                 exclamation_mark.to_token_stream(),
    //                 group1.to_token_stream(),
    //                 last_semicolon.to_token_stream(),
    //             ]
    //             .into_iter(),
    //         );
    //         eprintln!("output_each is {}", output_each);

    //         quote! {#output_each}
    //     })
    //     .collect::<proc_macro2::TokenStream>();
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
