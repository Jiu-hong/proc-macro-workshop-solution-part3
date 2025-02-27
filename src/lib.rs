#![feature(proc_macro_diagnostic)]
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Group, TokenTree};
use quote::{ToTokens, quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, Ident, Token, Type, Visibility, parse_macro_input, token};
// use syn::{Field, Ident, ItemEnum, Result, Token, braced, token};

mod impl_body;
use impl_body::myimpl;

///     lazy_static! {
///         static ref USERNAME: Regex = Regex::new("^[a-z0-9_-]{3,16}$").unwrap();
///     }

#[derive(Debug)]
struct SeqStruct {
    name: Ident,
    from: syn::LitInt,
    to: syn::LitInt,
    inner: proc_macro2::TokenStream,
}
// seq!(N in 0..8 {
//     // nothing
// });

impl Parse for SeqStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let from: syn::LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let to: syn::LitInt = input.parse()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let inner: proc_macro2::TokenStream = content.parse()?;
        Ok(SeqStruct {
            name,
            from,
            to,
            inner,
        })
    }
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

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqStruct);
    let name = input.name;
    let from = input.from;
    let from_int: u64 = from.base10_parse().unwrap();
    let to = input.to;
    let to_int: u64 = to.base10_parse().unwrap();
    let inner = input.inner;

    let output = (from_int..to_int)
        .map(|x| {
            let new_inner = update_group_ident(inner.clone(), &name, x);
            quote! {#new_inner}
        })
        .collect::<proc_macro2::TokenStream>();

    output.into()
}
