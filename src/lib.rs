#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
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
struct SeqStruct {}
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
        // eprintln!("content is {:#?}", content);
        let inner: proc_macro2::TokenStream = content.parse()?;
        eprintln!("inner is {}", inner);
        Ok(SeqStruct {})
    }
}
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqStruct);
    println!("input is {:#?}", input);
    TokenStream::new()
}
