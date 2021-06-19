extern crate proc_macro;

mod lambda;

#[proc_macro]
pub fn lambda(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lambda::lambda(input.into()).into()
}

#[rustversion::since(1.53)]
#[proc_macro]
pub fn λ(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lambda::lambda(input.into()).into()
}
