//! Scala's Placeholder Syntax for Rust.
//!
//! ```
//! # let xs = vec![1, 2, 3, 4, 5];
//! use placeholder_closure::λ;
//!
//! let xs = xs.into_iter().map(λ!($ + 1)).collect::<Vec<_>>();
//! ```
//!
//! # How it works
//!
//! The [`λ!`] (or [`lambda!`]) macro replaces 0 or more `$` characters with closure arguments.
//!
//! ```
//! let f = λ!($ + 1);
//! ```
//!
//! will be:
//!
//! ```
//! let f = |__0| __0 + 1;
//! ```
//!
//! ## Constructing `move` closures
//!
//! You can also construct `move` closures.
//!
//! ```
//! # use placeholder_closure::λ;
//! fn dot<F: FnOnce(Y) -> Z, G: FnOnce(X) -> Y, X, Y, Z>(f: F, g: G) -> impl FnOnce(X) -> Z {
//!     λ!(move { f(g($)) })
//! }
//! ```
//!
//! [`λ!`]: ./macro.λ.html
//! [`lambda!`]: ./macro.lambda.html

extern crate proc_macro;

mod lambda;

/// An alias for [`λ!`].
///
/// [`λ!`]: ./macro.λ.html
#[proc_macro]
pub fn lambda(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lambda::lambda(input.into()).into()
}

/// Scala's Placeholder Syntax for Rust.
///
/// Available since Rust 1.53.
///
/// See the [crate level documentation] for details.
///
/// [crate level documentation]: ./index.html
#[rustversion::since(1.53)]
#[proc_macro]
pub fn λ(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lambda::lambda(input.into()).into()
}
