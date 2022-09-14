pub mod error;
pub mod hash;

use ::std::os::raw::c_int;
use std::ffi::CString;

use error::{ErrorMessageHolder, VWError};
use vowpalwabbit_sys::VW_STATUS_SUCCESS;

pub struct Workspace {
    workspace: *mut vowpalwabbit_sys::VWWorkspace,
}

impl Workspace {
    // TODO use a trait bound of something to make this a more flexible input, &str, &String, String etc
    pub fn new(args: &[String]) -> Result<Workspace, VWError> {
        let mut workspace: *mut vowpalwabbit_sys::VWWorkspace = std::ptr::null_mut();

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
            let res = vowpalwabbit_sys::VWWorkspaceInitialize(
                c_args.as_ptr(),
                c_args.len() as c_int,
                &mut workspace,
                error_message_holder.get_mut_ptr(),
            );

            if res != 0 {
                match error_message_holder.get() {
                    Some(message) => Err(VWError::Failure(message)),
                    None => Err(VWError::Failure("Unknown".to_string())),
                }
            } else {
                Ok(Workspace { workspace })
            }
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWWorkspaceDelete(self.workspace);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Workspace;

    #[test]
    fn create_workspace() {
        let args: Vec<String> = vec!["--version".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_ok());
    }

    #[test]
    fn create_workspace_with_invalid_option() {
        let args: Vec<String> = vec!["--not_real".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_err());
    }
}
