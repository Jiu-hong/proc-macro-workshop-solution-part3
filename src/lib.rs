#![feature(proc_macro_diagnostic)]
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Literal, Punct, Span, TokenTree};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Brace, RArrow};
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
    // inner: proc_macro2::TokenStream,
    fn_name: Token![fn],
    f_name: Ident,
    n_ident: Ident,
    group: Group,
    return_arrow: RArrow,
    return_type: Ident,
    fn_brace_token: Brace,
    inner_ident: Ident,
    inner_punct: Punct,
    inner_literal: Literal,
}
// seq!(N in 0..8 {
//     // nothing
// });

// seq!(N in 1..4 {
//     fn f~N () -> u64 {
//         N * 2
//     }
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
        let fn_name = content.parse::<Token![fn]>()?;
        let f_name: Ident = content.parse()?;
        content.parse::<Token![~]>()?;
        let n_ident: Ident = content.parse()?;
        let group: Group = content.parse()?;
        let return_arrow = content.parse::<Token![->]>()?;

        let return_type: Ident = content.parse()?;
        let content_inner;
        let fn_brace_token = syn::braced!(content_inner in content);
        let inner_ident: Ident = content_inner.parse()?;
        let inner_punct: Punct = content_inner.parse()?;
        let inner_literal: Literal = content_inner.parse()?;

        // let inner: proc_macro2::TokenStream = content.parse()?;
        Ok(SeqStruct {
            name,
            from,
            to,
            fn_name,
            f_name,
            n_ident,
            group,
            return_arrow,
            return_type,
            fn_brace_token,
            inner_ident,
            inner_punct,
            inner_literal,
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    eprintln!("input is {:#?}", input);
    let input = parse_macro_input!(input as SeqStruct);
    let name = input.name;
    let from = input.from;
    let from_int: u64 = from.base10_parse().unwrap();
    let to = input.to;
    let to_int: u64 = to.base10_parse().unwrap();
    // fn_name: Ident,
    let fn_name = input.fn_name;
    // f_name: Ident,
    let f_name = input.f_name;
    // n_ident: Ident,
    let n_ident = input.n_ident;
    // group: Group,
    let group = input.group;
    // punct_: Punct,
    let return_arrow = input.return_arrow;
    // return_type: Ident,
    let return_type = input.return_type;
    // fn_brace_token: Brace,
    let fn_brace_token = input.fn_brace_token;
    // inner_ident: Ident,
    let inner_ident = input.inner_ident;
    // inner_punct: Punct,
    let inner_punct = input.inner_punct;
    // inner_literal: Literal,
    let inner_literal = input.inner_literal;

    // eprintln!("inner is {:#?}", inner);
    eprintln!("name is {:#?}", name);
    eprintln!("from is {:#?}", from);
    eprintln!("to is {:#?}", to);
    eprintln!("fn_name is {:#?}", fn_name);
    eprintln!("f_name is {:#?}", f_name);
    eprintln!("n_ident is {:#?}", n_ident);
    eprintln!("group is {:#?}", group);
    eprintln!("return_arrow {:#?}", return_arrow);
    eprintln!("return_type is {:#?}", return_type);
    eprintln!("fn_brace_token is {:#?}", fn_brace_token);
    eprintln!("inner_ident is {:#?}", inner_ident);
    eprintln!("inner_punct is {:#?}", inner_punct);
    eprintln!("inner_literal is {:#?}", inner_literal);

    let fn_token_stream = fn_name.to_token_stream();
    eprintln!("fn_token_stream is {:#?}", fn_token_stream);

    let group_token = group.to_token_stream();
    let return_arrow_token: proc_macro2::TokenStream = return_arrow.to_token_stream();
    let return_type_token = return_type.to_token_stream();

    let output = (from_int..to_int)
        .map(|index| {
            // let f_name_ident = f_name;
            let fn_name = f_name.to_string() + &index.to_string();
            let f_name_ident = Ident::new(&fn_name.to_string(), f_name.span());

            // inner group
            let inner_ident_token = index.to_token_stream();
            let inner_punct_token = inner_punct.to_token_stream();
            let inner_literal_token = inner_literal.to_token_stream();
            let inner_group = Group::new(
                Delimiter::Brace,
                proc_macro2::TokenStream::from_iter(
                    [inner_ident_token, inner_punct_token, inner_literal_token].into_iter(),
                ),
            );

            let output_each: proc_macro2::TokenStream = proc_macro2::TokenStream::from_iter(
                [
                    fn_token_stream.clone(),
                    // f_token_stream.clone(), add number
                    f_name_ident.to_token_stream(),
                    group_token.clone(),
                    return_arrow_token.clone(),
                    return_type_token.clone(),
                    inner_group.into_token_stream(),
                ]
                .into_iter(),
            );
            quote! {#output_each}
        })
        .collect::<proc_macro2::TokenStream>();
    // eprintln!("output is {}", output_each);

    // quote! {#output_each}.into()
    output.into()
}
