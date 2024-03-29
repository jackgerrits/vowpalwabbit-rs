#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("generated.rs");

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn test_simple_init() {
        unsafe {
            let error_message = VWErrorMessageCreate();
            let mut workspace: *mut VWWorkspace = std::ptr::null_mut();
            let res = VWWorkspaceInitialize(std::ptr::null(), 0, &mut workspace, error_message);
            assert!(res == VW_STATUS_SUCCESS);
            VWWorkspaceDelete(workspace);
        }
    }

    #[test]
    fn test_failing_init() {
        let args: Vec<CString> = vec![CString::new("--non_existant").unwrap()];
        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const ::std::os::raw::c_char>>();

        unsafe {
            let error_message = VWErrorMessageCreate();
            let mut workspace: *mut VWWorkspace = std::ptr::null_mut();
            let res = VWWorkspaceInitialize(
                c_args.as_ptr(),
                c_args.len().try_into().unwrap(),
                &mut workspace,
                error_message,
            );
            assert!(res == VW_STATUS_FAIL);
        }
    }

    #[test]
    fn test_create_and_delete_error_message() {
        unsafe {
            let error_message = VWErrorMessageCreate();
            assert!(!error_message.is_null());
            let value_or_nullptr = VWErrorMessageGetValue(error_message);
            assert!(value_or_nullptr.is_null());
            VWErrorMessageDelete(error_message);
        }
    }
}
