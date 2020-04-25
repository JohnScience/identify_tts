use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use proc_macro_hack::proc_macro_hack;


// TODO: Switch out from proc_macro_hack and edit documentation when
// "error: procedural macros cannot be expanded to expressions" is resloved.
// note: see issue #54727 <https://github.com/rust-lang/rust/issues/54727> for more information
#[proc_macro_hack]
/// Returns the slice of str& representation of variants of TokenTree in the given token stream
///
/// # Arguments
/// 
/// * `input` - A TokenStream whose token trees we want to count
///
/// # Example
///
/// * None
/// 
pub fn identify_tts(input: TokenStream) -> TokenStream {
    let tt_names :Vec::<String> = input
        .into_iter()
        .map(|tt| match tt {
            TokenTree::Group(group) => format!("[group]: {}", group),
            TokenTree::Ident(ident) => format!("[ident]: {}", ident),
            TokenTree::Punct(punct) => format!("[punct]: {}", punct),
            TokenTree::Literal(literal) => format!("[literal]: {}", literal),
        })
        .collect();
    let expanded_tt  = quote!{[#(#tt_names),*]};
    TokenStream::from(expanded_tt)
}