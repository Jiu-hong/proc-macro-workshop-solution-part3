use crate::Original;

pub fn original_token_stream(original: Original) -> proc_macro2::TokenStream {
    original.inner
}
