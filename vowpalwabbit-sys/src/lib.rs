include!("generated.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_init() {
        unsafe {
            let mut workspace: *mut VWWorkspace = std::ptr::null_mut();
            let res = VWInitializeWorkspace(
                std::ptr::null(),
                0,
                &mut workspace,
                std::ptr::null_mut(),
            );
            assert!(res == 0);
            let res = VWFreeWorkspace(workspace, std::ptr::null_mut());
            assert!(res == 0);
        }
    }
}
