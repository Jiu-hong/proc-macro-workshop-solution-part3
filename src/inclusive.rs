use crate::InclusiveRange;
use proc_macro2::{Delimiter, Group, Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::token::{Comma, Enum};
use syn::{Token, parse_macro_input};

struct Element {
    enum_symbol: Enum,
    enum_name: Ident,
    variant_name: Ident,
    comma: Comma,
}

impl Parse for Element {
    fn parse(content: ParseStream) -> Result<Self> {
        let enum_symbol = content.parse::<Token![enum]>()?;
        let enum_name: Ident = content.parse()?;
        let inner_content;

        let _ = syn::braced!(inner_content in content);
        let _ = inner_content.parse::<Token![#]>()?;
        // group
        let innermost;
        let _ = syn::parenthesized!(innermost in inner_content);
        let variant_name: Ident = innermost.parse()?;
        let _ = innermost.parse::<Token![~]>()?;
        let _n_name: Ident = innermost.parse()?;
        let comma: Comma = innermost.parse::<Token![,]>()?;
        let _ = inner_content.parse::<Token![*]>()?;
        Ok(Element {
            enum_symbol,
            enum_name,
            variant_name,
            comma,
        })
    }
}
pub fn inclusive_range(
    element: InclusiveRange,
    from_int: u64,
    to_int: u64,
    inclusive_range_flag: bool,
) -> proc_macro::TokenStream {
    let input = element.inner.into();

    let output = parse_macro_input!(input as Element);
    let enum_symbol = output.enum_symbol;
    let enum_name = output.enum_name;
    let variant_name = output.variant_name;
    let comma = output.comma;

    let groups_inclusive = (from_int..=to_int).map(|index| {
        let combined_name = variant_name.to_string() + &index.to_string();

        let index_tokenstream: proc_macro2::TokenStream =
            Ident::new(&combined_name.to_string(), variant_name.span()).to_token_stream();

        let stream = proc_macro2::TokenStream::from_iter(
            [index_tokenstream, comma.to_token_stream()].into_iter(),
        );
        stream
    });
    let groups_non_inclusive = (from_int..to_int).map(|index| {
        let combined_name = variant_name.to_string() + &index.to_string();

        let index_tokenstream: proc_macro2::TokenStream =
            Ident::new(&combined_name.to_string(), variant_name.span()).to_token_stream();

        let stream = proc_macro2::TokenStream::from_iter(
            [index_tokenstream, comma.to_token_stream()].into_iter(),
        );
        stream
    });
    let group = if inclusive_range_flag {
        Group::new(Delimiter::Brace, TokenStream::from_iter(groups_inclusive))
    } else {
        Group::new(
            Delimiter::Brace,
            TokenStream::from_iter(groups_non_inclusive),
        )
    };

    let output = TokenStream::from_iter(
        [
            enum_symbol.to_token_stream(),
            enum_name.to_token_stream(),
            group.into_token_stream(),
        ]
        .into_iter(),
    );

    quote! {#output}.into()
}
