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
mod impl_body;
use impl_body::myimpl;

#[derive(Debug)]
struct SeqStruct {
    name: Ident,
    from: syn::LitInt,
    to: syn::LitInt,
    inner: proc_macro2::TokenStream,
}

struct Compile {}

struct PasteIdent {
    fn_symbol: Token![fn],
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

impl Parse for SeqStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let from: syn::LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let to: syn::LitInt = input.parse()?;
        let inner: proc_macro2::TokenStream = input.parse()?;

        // let inner: proc_macro2::TokenStream = content.parse()?;
        Ok(SeqStruct {
            name,
            from,
            to,
            inner,
        })
    }
}

impl Parse for PasteIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace_token = syn::braced!(content in input);
        let fn_symbol = content.parse::<Token![fn]>()?;
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
        Ok(PasteIdent {
            fn_symbol,
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

    eprintln!("name is {:#?}", name);
    eprintln!("from is {:#?}", from);
    eprintln!("to is {:#?}", to);

    // pase PasteIdent
    let inner: proc_macro::TokenStream = input.inner.into();
    let parsed_inner = parse_macro_input!(inner as PasteIdent);
    // fn_name: Ident,
    let fn_symbol = parsed_inner.fn_symbol;
    // f_name: Ident,
    let f_name = parsed_inner.f_name;
    // n_ident: Ident,
    let _n_ident = parsed_inner.n_ident;
    // group: Group,
    let group = parsed_inner.group;
    // punct_: Punct,
    let return_arrow = parsed_inner.return_arrow;
    // return_type: Ident,
    let return_type = parsed_inner.return_type;
    // fn_brace_token: Brace,
    let _fn_brace_token = parsed_inner.fn_brace_token;
    // inner_ident: Ident,
    let _inner_ident = parsed_inner.inner_ident;
    // inner_punct: Punct,
    let inner_punct = parsed_inner.inner_punct;
    // inner_literal: Literal,
    let inner_literal = parsed_inner.inner_literal;

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
                    fn_symbol.to_token_stream(),
                    f_name_ident.to_token_stream(),
                    group.to_token_stream(),
                    return_arrow.to_token_stream(),
                    return_type.to_token_stream(),
                    inner_group.to_token_stream(),
                ]
                .into_iter(),
            );
            quote! {#output_each}
        })
        .collect::<proc_macro2::TokenStream>();

    output.into()
}
