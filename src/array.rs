use crate::RepeatedElement;
use proc_macro2::{Delimiter, Group, Ident};
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    token::{Comma, PathSep},
};

// Proc::new(N),

struct Element {
    proc_name: Ident,
    path_sep: PathSep,
    new_name: Ident,
    comma: Comma,
}

impl Parse for Element {
    fn parse(content: ParseStream) -> Result<Self> {
        let content_inner;
        let _ = syn::bracketed!(content_inner in content);
        let _ = content_inner.parse::<Token![#]>()?;
        let content_inner_inner;
        let _ = syn::parenthesized!(content_inner_inner in content_inner);

        let proc_name: Ident = content_inner_inner.parse()?;
        let path_sep = content_inner_inner.parse::<Token![::]>()?;
        let new_name: Ident = content_inner_inner.parse()?;
        // group
        let content_innermost;
        let _ = syn::parenthesized!(content_innermost in content_inner_inner);
        let _n_name: Ident = content_innermost.parse()?;
        let comma = content_inner_inner.parse::<Token![,]>()?;
        let _ = content_inner.parse::<Token![*]>()?;

        Ok(Element {
            proc_name,
            path_sep,
            new_name,
            comma,
        })
    }
}

pub fn init_array(
    repeated_element: RepeatedElement,
    from_int: u64,
    to_int: u64,
) -> proc_macro::TokenStream {
    let input = repeated_element.inner.into();
    let output = parse_macro_input!(input as Element);
    let proc_name = output.proc_name;
    let path_sep = output.path_sep;
    let new_name = output.new_name;
    let comma = output.comma;

    let elements = (from_int as usize..to_int as usize).map(|index| {
        let group = Group::new(Delimiter::Parenthesis, index.to_token_stream());
        let element = [
            proc_name.to_token_stream(),
            path_sep.to_token_stream(),
            new_name.to_token_stream(),
            group.to_token_stream(),
            comma.to_token_stream(),
        ]
        .into_iter();
        proc_macro2::TokenStream::from_iter(element)
    });

    let group = Group::new(
        Delimiter::Bracket,
        proc_macro2::TokenStream::from_iter(elements),
    );

    quote! {#group}.into()
}
