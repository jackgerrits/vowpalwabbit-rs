# VowpalWabbit-sys-rs
[![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys)

This crate wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)'s C binding interface. It handles finding the installed VW binaries on the system and linking them into a Rust crate.

See [vowpalwabbit-rs](https://github.com/jackgerrits/vowpalwabbit-rs) for the Rust wrapper around the sys package. This is still a work in progress.

## Documentation
- [8.7.0](https://jackgerrits.com/vowpalwabbit-sys-rs/8_7_0/vowpalwabbit_sys/)
- [8.6.1](https://jackgerrits.com/vowpalwabbit-sys-rs/8_6_1/vowpalwabbit_sys/)

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
