use proc_macro::{Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

use quote::quote;
use syn::{parse::Parse, parse_macro_input, Token};


struct KeyValuePairs(
    Vec<(syn::Expr,syn::Expr)>
);

impl Parse for KeyValuePairs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut acc = vec![];
        loop {
            if input.is_empty() {
                break Ok(Self(acc));
            }
            let key: syn::Expr = input.parse()?;
            let _: Token![=>] = input.parse()?;
            let value: syn::Expr = input.parse()?;
            let _: syn::Token![,] = input.parse();

            acc.push((key,value))
        }
        
    }
}
/// works only with literals since i'm lazy
#[proc_macro]
pub fn btreemap(toks: TokenStream) -> TokenStream {
    let input = parse_macro_input!(toks as KeyValuePairs);

    let iter = input.0.iter().map(|(k,v)| {
        quote::quote!(
            btreemap.insert(#k,#v);
        )
    });

    let expr = quote::quote!(
        {
            let mut btreemap = std::collections::BTreeMap::new();
            #(#iter)*
        }
    );

    TokenStream::from(expr)
}
