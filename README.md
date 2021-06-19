# placeholder_closure

[![CI](https://github.com/qryxip/placeholder_closure/workflows/CI/badge.svg)](https://github.com/qryxip/placeholder_closure/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/placeholder_closure.svg)](https://crates.io/crates/placeholder_closure)
[![Crates.io](https://img.shields.io/crates/l/placeholder_closure.svg)](https://crates.io/crates/placeholder_closure)

Scala's Placeholder Syntax for Rust.

```rust
use placeholder_closure::λ;

let xs = xs.into_iter().map(λ!($ + 1)).collect::<Vec<_>>();

fn dot<F: FnOnce(Y) -> Z, G: FnOnce(X) -> Y, X, Y, Z>(f: F, g: G) -> impl FnOnce(X) -> Z {
    λ!(move { f(g($)) })
}
```

## Usage

See the [documentation on Docs.rs](https://docs.rs/placeholder_syntax).

## License

Licensed under [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
