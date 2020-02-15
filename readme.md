# VowpalWabbit-sys-rs
[![build](https://github.com/jackgerrits/vowpalwabbit-sys-rs/workflows/build/badge.svg?branch=master)](https://github.com/jackgerrits/vowpalwabbit-sys-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys)
[![Docs](https://docs.rs/vowpalwabbit-sys/badge.svg)](https://docs.rs/vowpalwabbit-sys)

This crate wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)'s C binding interface. This crate wraps all of the dependencies and builds from source on each platform. For details about how the dependencies are configured see [here](https://github.com/jackgerrits/vowpalwabbit-sys-rs/blob/master/dependencies.md).

See [vowpalwabbit-rs](https://github.com/jackgerrits/vowpalwabbit-rs) for the Rust wrapper around the sys package. This is still a work in progress.

The major and minor versions of this crate track that of the native VW library that is wraps. The patch version, though, may be out of sync due to the need to patch the crate out of sync with the native dependency. Starting at version `8.8.1+vw-v8.8.0` you can determine the version of Vowpal Wabbit that it wraps by looking at the associated SemVer metadata. In this case it is `vw-v8.8.0` indicating the wrapped Vowpal Wabbit version is `8.8.0`. 

## Example

The following is an example for a basic use case similar to command line driver mode. VW is initialized, an example run through the parser then prediction pipeline. Finally the example and VW object are finished.

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

## Troubleshooting

If the build fails with something like the following:
```
thread 'main' panicked at 'Unable to find libclang: "couldn\'t find any valid shared libraries matching: [\'clang.dll\', \'libclang.dll\'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"', src\libcore\result.rs:1188:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
You need to install LLVM as the header codegen requires it.

On Windows:
1. Download and install LLVM: http://releases.llvm.org/
2. Set the environment variable `LIBCLANG_PATH` to where `libclang.dll` or `clang.dll` is located
    - You can find the install location of LLVM by using `where clang`
