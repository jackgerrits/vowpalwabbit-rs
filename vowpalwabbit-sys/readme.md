# VowpalWabbit-sys-rs
[![build](https://github.com/jackgerrits/vowpalwabbit-rs/workflows/build/badge.svg?branch=master)](https://github.com/jackgerrits/vowpalwabbit-sys-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys)
[![Docs](https://docs.rs/vowpalwabbit-sys/badge.svg)](https://docs.rs/vowpalwabbit-sys)

This crate wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)'s C binding interface.

See the [vowpalwabbit crate](../vowpalwabbit) for the Rust wrapper around the sys package. This is still a work in progress.

The major and minor versions of this crate track that of the native VW library that is wraps. The patch version, though, may be out of sync due to the need to patch the crate out of sync with the native dependency. Starting at version `8.8.1+vw-v8.8.0` you can determine the version of Vowpal Wabbit that it wraps by looking at the associated SemVer metadata. In this case it is `vw-v8.8.0` indicating the wrapped Vowpal Wabbit version is `8.8.0`.

## How to regenerate the bindings

```sh
cargo install bindgen
bindgen binding/include/vw_rs_bindings/binding.hpp -o src/bindings.rs
```