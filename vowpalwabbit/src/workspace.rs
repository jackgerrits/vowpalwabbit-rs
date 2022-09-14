use std::{ffi::CString, os::raw::c_int};

use vowpalwabbit_sys::VW_STATUS_SUCCESS;

use crate::{
    error::{check_result, ErrorMessageHolder, VWError},
    example::Example,
};

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
            check_result!(res, error_message_holder);
            Ok(Workspace { workspace })
        }
    }

    fn get_ptr(&self) -> *const vowpalwabbit_sys::VWWorkspace {
        self.workspace
    }

    fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWWorkspace {
        self.workspace
    }

    pub fn get_example_from_pool(&mut self) -> Result<Example, VWError> {
        unsafe {
            // TODO check result
            let mut error_message_holder = ErrorMessageHolder::new();
            let mut example: *mut vowpalwabbit_sys::VWExample = std::ptr::null_mut();
            let res = vowpalwabbit_sys::VWWorkspaceGetPooledExample(
                self.get_mut_ptr(),
                &mut example,
                error_message_holder.get_mut_ptr(),
            );
            check_result!(res, error_message_holder);
            Ok(Example { example })
        }
    }

    pub fn return_example_to_pool(&mut self, mut example: Example) -> Result<(), VWError> {
        unsafe {
            // TODO check result
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWWorkspaceReturnPooledExample(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                error_message_holder.get_mut_ptr(),
            );
            check_result!(res, error_message_holder);

            // Since we explicitly gave ownership back to the workspace we dont want to drop it
            std::mem::forget(example);
            Ok(())
        }
    }

    pub fn learn(&mut self, example: &mut Example) -> Result<(), VWError> {
        unsafe {
            // TODO check result
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWWorkspaceLearn(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                error_message_holder.get_mut_ptr(),
            );
            check_result!(res, error_message_holder);
            Ok(())
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWWorkspaceDelete(self.get_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace::Workspace;

    #[test]
    fn create_workspace() {
        let args: Vec<String> = vec!["--quiet".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_ok());
    }

    #[test]
    fn create_workspace_with_invalid_option() {
        let args: Vec<String> = vec!["--not_real".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_err());
    }

    #[test]
    fn create_and_return_example_to_pool() {
        let args: Vec<String> = vec![];
        let mut workspace = Workspace::new(&args).unwrap();
        let mut _example1 = workspace.get_example_from_pool().unwrap();
        workspace.return_example_to_pool(_example1).unwrap();
    }
}
