#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::{Group, Literal, Punct};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Brace, RArrow};
use syn::{Ident, Token, parse_macro_input, token};
mod paste_ident;
use paste_ident::paste_ident_token_stream;
mod compile_error;
use compile_error::compile_token_stream;
mod original;
use original::original_token_stream;
mod secion;
use secion::repeat_section;
mod array;
use array::init_array;
mod inclusive;
use inclusive::inclusive_range;
mod spanmod;
use spanmod::ident_span;

#[derive(Debug)]
enum Item {
    Paste(PasteIdent),
    Comp(Compile),
    RepeatSection(RepeatSection),
    InitArray(RepeatedElement),
    Inclusive(InclusiveRange),
    IdentSpan(MissingIdent),
    Any(Original),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        // let lookahead = input.lookahead1();
        if input.peek2(Token![!]) {
            input.parse().map(Item::Comp)
        } else if input.peek(Token![fn]) && input.peek3(Token![~]) {
            input.parse().map(Item::Paste)
        } else if input.peek(Token![#]) && input.peek3(Token![enum]) {
            input.parse().map(Item::RepeatSection)
        } else if input.peek(token::Bracket) {
            input.parse().map(Item::InitArray)
        } else if input.peek(Token![enum]) {
            input.parse().map(Item::Inclusive)
        } else if input.peek(Token![fn]) {
            input.parse().map(Item::IdentSpan)
        } else {
            input.parse().map(Item::Any)
        }
    }
}

#[derive(Debug)]
struct SeqStruct {
    name: Ident,
    from: syn::LitInt,
    to: syn::LitInt,
    inner: proc_macro2::TokenStream,
    inclusive_range_flag: bool,
}

#[derive(Debug)]
struct RepeatSection {
    inner: proc_macro2::TokenStream,
}

#[derive(Debug)]
struct RepeatedElement {
    inner: proc_macro2::TokenStream,
}

#[derive(Debug)]
struct MissingIdent {
    inner: proc_macro2::TokenStream,
}

impl Parse for MissingIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(MissingIdent { inner })
    }
}
#[derive(Debug)]
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

#[derive(Debug)]

struct Compile {
    inner: proc_macro2::TokenStream,
}

#[derive(Debug)]
struct Original {
    inner: proc_macro2::TokenStream,
}

impl Parse for Original {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(Original { inner })
    }
}

impl Parse for RepeatSection {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(RepeatSection { inner })
    }
}

impl Parse for RepeatedElement {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(RepeatedElement { inner })
    }
}

#[derive(Debug)]
struct InclusiveRange {
    inner: proc_macro2::TokenStream,
}
impl Parse for InclusiveRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(InclusiveRange { inner })
    }
}
// compile_error!(concat!("error number ", stringify!(N)));
impl Parse for Compile {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner: proc_macro2::TokenStream = input.parse()?;
        Ok(Compile { inner })
    }
}

impl Parse for SeqStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let from: syn::LitInt = input.parse()?;
        // input.parse::<Token![..]>()?;
        let inclusive_range_flag = if input.peek(Token![..=]) {
            input.parse::<Token![..=]>()?;
            true
        } else {
            input.parse::<Token![..]>()?;
            false
        };
        let to: syn::LitInt = input.parse()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let inner: proc_macro2::TokenStream = content.parse()?;
        Ok(SeqStruct {
            name,
            from,
            to,
            inner,
            inclusive_range_flag,
        })
    }
}

impl Parse for PasteIdent {
    fn parse(content: ParseStream) -> Result<Self> {
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
    let input = parse_macro_input!(input as SeqStruct);
    let name = input.name;
    let from = input.from;
    let from_int: u64 = from.base10_parse().unwrap();
    let to = input.to;
    let to_int: u64 = to.base10_parse().unwrap();
    let inclusive_range_flag = input.inclusive_range_flag;
    //
    // PasteIdent
    let inner: proc_macro::TokenStream = input.inner.into();

    let output = match parse_macro_input!(inner as Item) {
        Item::Paste(paste_ident) => paste_ident_token_stream(paste_ident, name, from_int, to_int),
        Item::Comp(compile) => compile_token_stream(compile, name, from_int, to_int),
        Item::RepeatSection(repeated_section) => {
            repeat_section(repeated_section, from_int, to_int).into()
        }
        Item::InitArray(repeated_element) => init_array(repeated_element, from_int, to_int).into(),
        Item::Inclusive(element) => {
            inclusive_range(element, from_int, to_int, inclusive_range_flag).into()
        }
        Item::Any(original) => original_token_stream(original),
        Item::IdentSpan(missing_ident) => ident_span(missing_ident, from_int, to_int).into(),
    };

    output.into()
}
