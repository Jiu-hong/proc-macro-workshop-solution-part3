use crate::MissingIdent;

use proc_macro2::{Delimiter, Group, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    token::{Eq, Fn, Let, Semi, Underscore},
};

struct Element {
    fn_kw: Fn,
    main_name: Ident,

    let_name: Let,
    under_score: Underscore,
    equal: Eq,
    missing_name: Ident,
    semicolon: Semi,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        // let a = input.parse()?;
        let fn_kw = input.parse::<Token![fn]>()?;
        let main_name: Ident = input.parse()?;

        let _content;
        let _ = syn::parenthesized!(_content in input);
        let inner_content;
        let _ = syn::braced!(inner_content in input);
        // let let_name: Ident = inner_content.parse()?;
        let let_name = inner_content.parse::<Token![let]>()?;
        let under_score = inner_content.parse::<Token![_]>()?;
        let equal = inner_content.parse::<Token![=]>()?;
        let missing_name: Ident = inner_content.parse()?;
        let _ = inner_content.parse::<Token![~]>()?;
        let _N: Ident = inner_content.parse()?;
        let semicolon = inner_content.parse::<Token![;]>()?;

        Ok(Element {
            fn_kw,
            main_name,

            let_name,
            under_score,
            equal,
            missing_name,
            semicolon,
        })
    }
}

pub fn ident_span(
    missing_ident: MissingIdent,
    from_int: u64,
    to_int: u64,
) -> proc_macro::TokenStream {
    let input = missing_ident.inner.into();

    let output = parse_macro_input!(input as Element);

    let elements = (from_int..to_int).map(|index| {
        let inner_function_tokenstream = output.let_name.to_token_stream();
        let under_score_tokenstream = output.under_score.to_token_stream();
        let equal_tokenstream = output.equal.to_token_stream();
        let missing_tokenstream = Ident::new(
            &(output.missing_name.to_string() + &index.to_string()),
            output.missing_name.span(),
        )
        .to_token_stream();
        let semicolon_tokenstream = output.semicolon.to_token_stream();
        TokenStream::from_iter(
            [
                output.fn_kw.to_token_stream(),
                output.main_name.to_token_stream(),
                Group::new(Delimiter::Parenthesis, TokenStream::new()).to_token_stream(),
                Group::new(
                    Delimiter::Brace,
                    TokenStream::from_iter(
                        [
                            inner_function_tokenstream,
                            under_score_tokenstream,
                            equal_tokenstream,
                            missing_tokenstream,
                            semicolon_tokenstream,
                        ]
                        .into_iter(),
                    ),
                )
                .to_token_stream(),
            ]
            .into_iter(),
        )
    });

    let output_stream = TokenStream::from_iter(elements);

    quote! {#output_stream}.into()
}
