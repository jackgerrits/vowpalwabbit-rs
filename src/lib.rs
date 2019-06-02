#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Rust bindings for the [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit) C-binding surface.
//! 
//! __Note:__ Currently this crate only supports discovering VW through `pkg-config`, therefore VowpalWabbit 
//! must be installed using `make install` for this crate to be able to find it and link to it. This
//! crate is not yet supported on Windows, once VW supports Vcpkg then this crate will use that to discover it.
//! 
//! In order to get the library installed:
//! 1. Clone [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)
//! 2. `mkdir build`
//! 3. `cd build`
//! 4. `make -j 8`
//! 5. `make install`
//!
//! ### Example
//! The following is an example for a basic usecase similar to command line driver mode. VW is initialized, an example run through the parser then prediction pipeline. Finally the example and VW object are finished.
//! ```
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
