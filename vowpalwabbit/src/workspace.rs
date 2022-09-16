use std::{ffi::CString, os::raw::c_int};

use vowpalwabbit_sys::{size_t, VW_STATUS_SUCCESS};

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

    pub fn parse_decision_service_json(&mut self, content: &str) -> Result<Vec<Example>, VWError> {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let mut multi_ex_handle = vowpalwabbit_sys::VWMultiExCreate();
            let res = vowpalwabbit_sys::VWWorkspaceParseDSJson(
                self.get_mut_ptr(),
                content.as_ptr() as *const i8,
                content.len().try_into().unwrap(),
                multi_ex_handle,
                error_message_holder.get_mut_ptr(),
            );
            check_result!(res, error_message_holder);

            let mut result_vec: Vec<Example> = Vec::new();
            let length: size_t = vowpalwabbit_sys::VWMultiGetLength(multi_ex_handle);
            for i in 0..length {
                let mut example: *mut vowpalwabbit_sys::VWExample = std::ptr::null_mut();
                let res = vowpalwabbit_sys::VWMultiGetExample(
                    multi_ex_handle,
                    &mut example,
                    i,
                    error_message_holder.get_mut_ptr(),
                );
                check_result!(res, error_message_holder);
                result_vec.push(Example { example });
            }
            // TODO: wrap these into struct so that if an early exit happens above then we dont leak
            vowpalwabbit_sys::VWMultiClear(multi_ex_handle);
            vowpalwabbit_sys::VWMultiExDelete(multi_ex_handle);
            Ok(result_vec)
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
    use crate::workspace::{self, Workspace};

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
    fn parse_dsjson() {
        let args: Vec<String> = vec!["--cb_explore_adf".to_owned()];
        let mut workspace = Workspace::new(&args).unwrap();
        let examples = workspace.parse_decision_service_json(r#"{"_label_cost":-0.0,"_label_probability":0.05000000074505806,"_label_Action":4,"_labelIndex":3,"o":[{"v":0.0,"EventId":"13118d9b4c114f8485d9dec417e3aefe","ActionTaken":false}],"Timestamp":"2021-02-04T16:31:29.2460000Z","Version":"1","EventId":"13118d9b4c114f8485d9dec417e3aefe","a":[4,2,1,3],"c":{"FromUrl":[{"timeofday":"Afternoon","weather":"Sunny","name":"Cathy"}],"_multi":[{"_tag":"Cappucino","i":{"constant":1,"id":"Cappucino"},"j":[{"type":"hot","origin":"kenya","organic":"yes","roast":"dark"}]},{"_tag":"Cold brew","i":{"constant":1,"id":"Cold brew"},"j":[{"type":"cold","origin":"brazil","organic":"yes","roast":"light"}]},{"_tag":"Iced mocha","i":{"constant":1,"id":"Iced mocha"},"j":[{"type":"cold","origin":"ethiopia","organic":"no","roast":"light"}]},{"_tag":"Latte","i":{"constant":1,"id":"Latte"},"j":[{"type":"hot","origin":"brazil","organic":"no","roast":"dark"}]}]},"p":[0.05,0.05,0.05,0.85],"VWState":{"m":"ff0744c1aa494e1ab39ba0c78d048146/550c12cbd3aa47f09fbed3387fb9c6ec"},"_original_label_cost":-0.0}"#).unwrap();
    }

    #[test]
    fn parse_invalid_dsjson() {
        let args: Vec<String> = vec!["--cb_explore_adf".to_owned()];
        let mut workspace = Workspace::new(&args).unwrap();
        let maybe_examples = workspace.parse_decision_service_json(r#"{"unclosed}"#);
        assert!(maybe_examples.is_err());
    }
}
