# VowpalWabbit-sys-rs
[![build](https://github.com/jackgerrits/vowpalwabbit-sys-rs/workflows/build/badge.svg?branch=master)](https://github.com/jackgerrits/vowpalwabbit-sys-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys)
[![Docs](https://docs.rs/vowpalwabbit-sys/badge.svg)](https://docs.rs/vowpalwabbit-sys)

This crate wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)'s C binding interface. This crate wraps all of the dependencies and builds from source on each platform. For details about how the dependencies are configured see [here](https://github.com/jackgerrits/vowpalwabbit-sys-rs/blob/master/dependencies.md).W

See [vowpalwabbit-rs](https://github.com/jackgerrits/vowpalwabbit-rs) for the Rust wrapper around the sys package. This is still a work in progress.

## Example

The following is an example for a basic usecase similar to command line driver mode. VW is initialized, an example run through the parser then prediction pipeline. Finally the example and VW object are finished.

```rust
use std::ffi::CString;

unsafe {
  let command_line_str = CString::new("--quiet").unwrap();
  let vw_handle = vowpalwabbit_sys::VW_InitializeA(command_line_str.as_ptr());
  let example_str =
    CString::new("1 | test example=1").unwrap();
  let example_handle = vowpalwabbit_sys::VW_ReadExampleA(vw_handle, example_str.as_ptr());

  vowpalwabbit_sys::VW_Predict(vw_handle, example_handle);
  vowpalwabbit_sys::VW_Learn(vw_handle, example_handle);
  vowpalwabbit_sys::VW_FinishExample(vw_handle, example_handle);
  vowpalwabbit_sys::VW_Finish(vw_handle);
}
```
