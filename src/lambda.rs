use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{quote, ToTokens as _};
use std::iter;

pub(crate) fn lambda(input: TokenStream) -> TokenStream {
    let (movability, input) = movability(input);
    let mut args = vec![];
    let body = replace_dollars(input, &mut args);
    quote!(#movability |#(#args),*| #body)
}

fn movability(input: TokenStream) -> (TokenStream, TokenStream) {
    let input = input.into_iter().collect::<Vec<_>>();
    if let [TokenTree::Ident(ident), TokenTree::Group(group)] = &*input {
        if ident == "move" && group.delimiter() == Delimiter::Brace {
            return (quote!(#ident), group.stream());
        }
    }
    (quote!(), input.into_iter().collect())
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
