#![feature(proc_macro_diagnostic)]
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::Group;
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
        eprintln!("name is {}", name);
        input.parse::<Token![in]>()?;
        let from: syn::LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let to: syn::LitInt = input.parse()?;
        eprintln!("to is {}", to);
        let content;
        let brace_token = syn::braced!(content in input);
        eprintln!("content is {}", content);
        let inner: proc_macro2::TokenStream = content.parse()?;
        eprintln!("inner is {}", inner);
        Ok(SeqStruct {
            name,
            from,
            to,
            inner,
        })
    }
}

fn get_group_ident(group: &Group) {
    let a = group.stream();
    if a.to_string() == "N" {
        let new_token = proc_macro2::TokenStream::from_str(&x.to_string()).unwrap();
        // replace group's name with specific number
        let d = group.delimiter();
        let new_group = Group::new(d, new_token.clone());
        token = proc_macro2::TokenTree::Group(new_group);
    }
    //final symbol
    //recursive
    else {
        for token in a {
            // a.into_iter().map(|mut token| {
            if let proc_macro2::TokenTree::Group(ref group) = token {
                get_group_ident(group)
            }
        }
    }
}
// [x*2 for x in range(5)]
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqStruct);
    println!("input is {:#?}", input);
    let name = input.name;
    let from = input.from;
    let from_token_stream = from.to_token_stream();
    let from_int: u64 = from.base10_parse().unwrap();
    let to = input.to;
    let to_token_stream = to.to_token_stream();
    let to_int: u64 = to.base10_parse().unwrap();
    let mut inner = input.inner;

    // let result: Vec<TokenStream> = new_inner_iter.collect();
    let output = (from_int..to_int).map(|x| {
        let new_inner_iter = inner.clone().into_iter().map(|mut token| {
            eprintln!("token is {:#?}", token);
            if let proc_macro2::TokenTree::Group(ref group) = token {
                get_group_ident(group);
                let a: proc_macro2::TokenStream = group.stream();
                eprintln!("a is {:#?}", a.to_string());
                if a.to_string() == name.to_string() {
                    let new_token = proc_macro2::TokenStream::from_str(&x.to_string()).unwrap();
                    // replace group's name with specific number
                    let d = group.delimiter();
                    let new_group = Group::new(d, new_token.clone());
                    token = proc_macro2::TokenTree::Group(new_group);
                }
            }
            return token;
        });
        let new_inner = proc_macro2::TokenStream::from_iter(new_inner_iter);
        eprintln!("new_inner: {:#?}", new_inner);
        println!("number is {}", x);
        quote! {#new_inner}
    });

    // eprintln!("output is {}", output);
    let a = output.collect::<proc_macro2::TokenStream>();
    // let a = inner.parse()?;
    eprintln!("output is: {:#?}", a);
    a.into()
}
