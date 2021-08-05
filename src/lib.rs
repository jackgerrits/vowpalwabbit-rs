#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// The doctest must be turned off due to this issue: https://github.com/rust-lang/cargo/issues/8531

//! Rust bindings for the [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit) C-binding surface.
//!
//! ### Example
//! The following is an example for a basic usecase similar to command line driver mode. VW is initialized, an example run through the parser then prediction pipeline. Finally the example and VW object are finished
//! ```no_run
//! use std::ffi::CString;
//!
//! unsafe {
//!   let command_line_str = CString::new("--quiet").unwrap();
//!   let vw_handle = vowpalwabbit_sys::VW_InitializeA(command_line_str.as_ptr());
//!   let example_str =
//!     CString::new("1 | test example=1").unwrap();
//!   let example_handle = vowpalwabbit_sys::VW_ReadExampleA(vw_handle, example_str.as_ptr());
//!
//!   vowpalwabbit_sys::VW_Predict(vw_handle, example_handle);
//!   vowpalwabbit_sys::VW_Learn(vw_handle, example_handle);
//!   vowpalwabbit_sys::VW_FinishExample(vw_handle, example_handle);
//!   vowpalwabbit_sys::VW_Finish(vw_handle);
//! }
//! ```
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_initialize_and_parse_learn_example() {
        unsafe {
            let command_line_str = CString::new("--quiet").unwrap();
            let vw_handle = VW_InitializeA(command_line_str.as_ptr());
            let example_str = CString::new("1 | test example=1").unwrap();
            let example_handle = VW_ReadExampleA(vw_handle, example_str.as_ptr());
            assert_eq!(VW_GetLabel(example_handle), 1.0);

            VW_Predict(vw_handle, example_handle);
            VW_Learn(vw_handle, example_handle);
            VW_FinishExample(vw_handle, example_handle);
            VW_Finish(vw_handle);
        }
    }
}
