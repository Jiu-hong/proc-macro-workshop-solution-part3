use crate::PasteIdent;
use proc_macro2::{Delimiter, Group, Ident};
use quote::{ToTokens, quote};
pub fn paste_ident_token_stream(
    paste_ident: PasteIdent,
    name: Ident,
    from_int: u64,
    to_int: u64,
) -> proc_macro2::TokenStream {
    let fn_symbol = paste_ident.fn_symbol;
    // f_name: Ident,
    let f_name = paste_ident.f_name;
    // n_ident: Ident,
    let _n_ident = paste_ident.n_ident;
    // group: Group,
    let group = paste_ident.group;
    // punct_: Punct,
    let return_arrow = paste_ident.return_arrow;
    // return_type: Ident,
    let return_type = paste_ident.return_type;
    // fn_brace_token: Brace,
    let _fn_brace_token = paste_ident.fn_brace_token;
    // inner_ident: Ident,
    let _inner_ident = paste_ident.inner_ident;
    // inner_punct: Punct,
    let inner_punct = paste_ident.inner_punct;
    // inner_literal: Literal,
    let inner_literal = paste_ident.inner_literal;

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
    output
}
