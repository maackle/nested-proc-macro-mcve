use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn noop(_attr: TokenStream, code: TokenStream) -> TokenStream {
    code
}

#[proc_macro_attribute]
pub fn quoter(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let mut p = Parser;
    let output = syn::fold::fold_item(&mut p, syn::parse(code).unwrap());
    TokenStream::from(quote! {#output})
}

struct Parser;

impl syn::fold::Fold for Parser {
    fn fold_item_fn(&mut self, i: ItemFn) -> ItemFn {
        i
    }
}
