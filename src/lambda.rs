use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens as _};
use std::iter;

pub(crate) fn lambda(input: TokenStream) -> TokenStream {
    let mut args = vec![];
    let body = replace_dollars(input, &mut args);
    quote!(|#(#args),*| #body)
}

fn replace_dollars(input: TokenStream, args: &mut Vec<syn::Ident>) -> TokenStream {
    let mut body = TokenStream::new();
    for tt in input {
        match tt {
            TokenTree::Group(group) => body.extend(iter::once(TokenTree::Group(
                proc_macro2::Group::new(group.delimiter(), replace_dollars(group.stream(), args)),
            ))),
            TokenTree::Punct(punct) if punct.as_char() == '$' => {
                let arg = syn::Ident::new(&format!("__{}", args.len()), punct.span());
                body.extend(arg.to_token_stream());
                args.push(arg);
            }
            tt => body.extend(iter::once(tt)),
        }
    }
    body
}
