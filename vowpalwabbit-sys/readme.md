# VowpalWabbit-sys
[![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys)
[![Docs](https://docs.rs/vowpalwabbit-sys/badge.svg)](https://docs.rs/vowpalwabbit-sys)

This crate wraps the DLL defined in the [binding directory](../binding).

See the [vowpalwabbit crate](../vowpalwabbit) for the Rust wrapper around the sys package.

## How to regenerate the bindings

When the DLL interface changes bindgen needs to be run to regenerate [`src/generated.rs`](src/generated.rs)

```sh
cargo install bindgen
bindgen ../binding/include/vw_rs_bindings/bindings.hpp -o src/generated.rs
```