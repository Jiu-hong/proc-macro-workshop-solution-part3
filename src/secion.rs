use crate::RepeatSection;
use proc_macro2::{Delimiter, Group, Ident};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    token::{Comma, Enum, Pound},
};

struct Section {
    feature_symbol: Pound, //keep
    features: Group,       //keep
    enum_kw: Enum,         //keep
    enum_name: Ident,      //keep
    inner_sentence: InnerSentence,
}

struct InnerSentence {
    inner_irq: Ident,
    comma: Comma,
}

impl Parse for InnerSentence {
    fn parse(content: ParseStream) -> Result<Self> {
        let inner_irq: Ident = content.parse()?;
        let _ = content.parse::<Token![~]>()?;
        let inner_n: Ident = content.parse()?;
        let comma = content.parse::<Token![,]>()?;
        Ok(InnerSentence { inner_irq, comma })
    }
}

impl Parse for Section {
    fn parse(content: ParseStream) -> Result<Self> {
        let feature_symbol = content.parse::<Token![#]>()?;
        let features: Group = content.parse()?;
        let enum_kw = content.parse::<Token![enum]>()?;
        let enum_name: Ident = content.parse()?;
        let content_inner;
        let _ = syn::braced!(content_inner in content);
        let _ = content_inner.parse::<Token![#]>()?;
        let content_inner_inner;
        let _ = syn::parenthesized!(content_inner_inner in content_inner);

        let inner_sentence: InnerSentence = content_inner_inner.parse()?;
        let _ = content_inner.parse::<Token![*]>()?;

        Ok(Section {
            feature_symbol,
            features,
            enum_kw,
            enum_name,
            inner_sentence,
        })
    }
}

pub fn repeat_section(
    section: RepeatSection,
    from_int: u64,
    to_int: u64,
) -> proc_macro::TokenStream {
    let input = section.inner.into();
    let output = parse_macro_input!(input as Section);

    let feature_symbol = output.feature_symbol;
    let features = output.features;
    let enum_kw = output.enum_kw;
    let enum_name = output.enum_name;
    let inner_sentence = output.inner_sentence;

    let repeats = (from_int..to_int).map(|index| {
        let irq_number = format_ident!("{}{}", inner_sentence.inner_irq, index.to_string());
        proc_macro2::TokenStream::from_iter(
            [
                irq_number.to_token_stream(),
                inner_sentence.comma.to_token_stream(),
            ]
            .into_iter(),
        )
    });

    let group = Group::new(
        Delimiter::Brace,
        proc_macro2::TokenStream::from_iter(repeats),
    );

    let output = proc_macro2::TokenStream::from_iter(
        [
            feature_symbol.to_token_stream(),
            features.to_token_stream(),
            enum_kw.to_token_stream(),
            enum_name.to_token_stream(),
            group.to_token_stream(),
        ]
        .into_iter(),
    );

    return quote! {#output}.into();
}
