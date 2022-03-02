pub mod hash;

use ::std::os::raw::c_int;
use std::ffi::CStr;
use std::ffi::CString;

use thiserror::Error;

use vowpalwabbit_sys::bindings::VW_STATUS_SUCCESS;

#[derive(Error, Debug)]
pub enum VWError {
    #[error("Generic failure")]
    Failure(String),
}

struct ErrorMessageHolder {
    error_string: *const ::std::os::raw::c_char,
}

impl ErrorMessageHolder {
    fn new() -> Self {
        ErrorMessageHolder {
            error_string: std::ptr::null(),
        }
    }

    fn get_mut_ptr(&mut self) -> *mut *const ::std::os::raw::c_char {
        &mut self.error_string
    }

    fn to_string(&self) -> Option<String> {
        if !self.error_string.is_null() {
            unsafe {
                Some(
                    CStr::from_ptr(self.error_string)
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        } else {
            None
        }
    }
}

impl Drop for ErrorMessageHolder {
    fn drop(&mut self) {
        unsafe {
            let res = vowpalwabbit_sys::bindings::VWFreeErrorMessage(self.error_string);
            if res != VW_STATUS_SUCCESS {
                panic!("Error while dropping error message");
            }
        }
    }
}

pub struct Workspace {
    workspace: *mut vowpalwabbit_sys::bindings::VWWorkspace,
}

impl Workspace {
    pub fn new(args: &[String]) -> Result<Workspace, VWError> {
        let mut workspace: *mut vowpalwabbit_sys::bindings::VWWorkspace = std::ptr::null_mut();

        // let mut x: vowpalwabbit_sys::bindings::VWWorkspace = vowpalwabbit_sys::bindings::VWWorkspace;

        let args = args
            .iter()
            .map(|arg| CString::new(arg.clone()).unwrap())
            .collect::<Vec<CString>>();

        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const ::std::os::raw::c_char>>();

        let mut error_message_holder = ErrorMessageHolder::new();
        unsafe {
            let res = vowpalwabbit_sys::bindings::VWInitializeWorkspace(
                c_args.as_ptr(),
                c_args.len() as c_int,
                &mut workspace,
                error_message_holder.get_mut_ptr(),
            );

            if res != 0 {
                match error_message_holder.to_string() {
                    Some(message) => Err(VWError::Failure(message)),
                    None => Err(VWError::Failure("Unknown".to_string())),
                }
            } else {
                Ok(Workspace { workspace })
            }
        }
    }

    pub fn run_driver(&mut self) -> Result<(), VWError> {
        let mut error_message_holder = ErrorMessageHolder::new();
        unsafe {
            let res = vowpalwabbit_sys::bindings::VWRunDriver(self.workspace, error_message_holder.get_mut_ptr());
             if res != 0 {
                match error_message_holder.to_string() {
                    Some(message) => Err(VWError::Failure(message)),
                    None => Err(VWError::Failure("Unknown".to_string())),
                }
            } else {
                Ok(())
        }
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            let mut error_string: *const ::std::os::raw::c_char = std::ptr::null();
            let res = vowpalwabbit_sys::bindings::VWFreeWorkspace(self.workspace, &mut error_string);
            if res != VW_STATUS_SUCCESS {
                let message = if !error_string.is_null() {
                    CStr::from_ptr(error_string).to_string_lossy().into_owned()
                } else {
                    "Unknown".to_string()
                };
                panic!("Error while dropping Workspace: {}", message);
            }
        }
    }
}
