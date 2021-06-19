use proc_macro2::{Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens as _};
use std::{iter, mem};

pub(crate) fn lambda(input: TokenStream) -> TokenStream {
    let (mut args, mut body) = (vec![], TokenStream::new());
    replace_dollars(input, &mut args, &mut body, &mut 0);
    quote!(|#(#args),*| #body)
}

fn replace_dollars(
    input: TokenStream,
    args: &mut Vec<syn::Ident>,
    body: &mut TokenStream,
    n: &mut u32,
) {
    for tt in input {
        match tt {
            TokenTree::Group(group) => replace_dollars(group.to_token_stream(), args, body, n),
            TokenTree::Punct(punct)
                if punct.as_char() == '$' && punct.spacing() == Spacing::Alone =>
            {
                let arg = syn::Ident::new(&name(n), punct.span());
                body.extend(arg.to_token_stream());
                args.push(arg);
            }
            tt => body.extend(iter::once(tt)),
        }
    }

    fn name(n: &mut u32) -> String {
        let n_next = *n + 1;
        format!("__{}", mem::replace(n, n_next))
    }
}
