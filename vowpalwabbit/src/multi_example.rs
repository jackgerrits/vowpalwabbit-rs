use std::convert::TryInto;

use vowpalwabbit_sys::VW_STATUS_SUCCESS;

use crate::{
    error::{check_panic, check_return, ErrorMessageHolder, VWError},
    example::{Example, RawExample},
};

pub struct MultiExample {
    pub(crate) multi_example: *mut vowpalwabbit_sys::VWMultiEx,
}

impl Drop for MultiExample {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWMultiExDelete(self.get_mut_ptr());
        }
    }
}

pub struct RawMultiExample {
    pub(crate) multi_example: *mut vowpalwabbit_sys::VWMultiEx,
}

unsafe impl Send for MultiExample {}
unsafe impl Sync for MultiExample {}
unsafe impl Send for RawMultiExample {}
unsafe impl Sync for RawMultiExample {}

impl RawMultiExample {
    pub fn new() -> RawMultiExample {
        unsafe {
            RawMultiExample {
                multi_example: vowpalwabbit_sys::VWMultiExCreate(),
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            vowpalwabbit_sys::VWMultiGetLength(self.get_ptr())
                .try_into()
                .unwrap()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_ptr(&self) -> *const vowpalwabbit_sys::VWMultiEx {
        self.multi_example
    }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWMultiEx {
        self.multi_example
    }

    pub fn extract_at(&mut self, index: usize) -> Option<RawExample> {
        unsafe {
            let mut example_ptr: *mut vowpalwabbit_sys::VWExample = std::ptr::null_mut();
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiReleaseExampleAt(
                self.get_mut_ptr(),
                &mut example_ptr,
                index.try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            // TODO do bounds check here instead?
            if res != VW_STATUS_SUCCESS {
                return None;
            }
            Some(RawExample {
                example: example_ptr,
            })
        }
    }

    pub fn push_example(&mut self, mut example: RawExample) {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiInsertExampleAt(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                self.len().try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            check_panic!(res, error_message_holder);
            example.release();
        }
    }

    pub fn insert_example_at(
        &mut self,
        mut example: RawExample,
        index: usize,
    ) -> Result<(), VWError> {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiInsertExampleAt(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                index.try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            example.release();
            check_return!(res, error_message_holder);
            Ok(())
        }
    }
}

impl MultiExample {
    pub fn len(&self) -> usize {
        unsafe {
            vowpalwabbit_sys::VWMultiGetLength(self.get_ptr())
                .try_into()
                .unwrap()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_ptr(&self) -> *const vowpalwabbit_sys::VWMultiEx {
        self.multi_example
    }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWMultiEx {
        self.multi_example
    }

    pub fn extract_at(&mut self, index: usize) -> Option<Example> {
        unsafe {
            let mut example_ptr: *mut vowpalwabbit_sys::VWExample = std::ptr::null_mut();
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiReleaseExampleAt(
                self.get_mut_ptr(),
                &mut example_ptr,
                index.try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            // TODO do bounds check here instead?
            if res != VW_STATUS_SUCCESS {
                return None;
            }
            Some(Example {
                example: example_ptr,
            })
        }
    }

    pub fn push_example(&mut self, mut example: Example) {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiInsertExampleAt(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                self.len().try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            check_panic!(res, error_message_holder);
            example.release();
        }
    }

    pub fn insert_example_at(&mut self, mut example: Example, index: usize) -> Result<(), VWError> {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWMultiInsertExampleAt(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                index.try_into().unwrap(),
                error_message_holder.get_mut_ptr(),
            );
            example.release();
            check_return!(res, error_message_holder);
            Ok(())
        }
    }
}

impl Default for RawMultiExample {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RawMultiExample {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWMultiExDelete(self.get_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::example::RawExample;

    use super::RawMultiExample;

    #[test]
    fn create_multi_example() {
        let _ = RawMultiExample::new();
    }

    #[test]
    fn insert_into_multi_example() {
        let mut multi_ex = RawMultiExample::new();
        multi_ex.push_example(RawExample::new());
        multi_ex.push_example(RawExample::new());
        multi_ex.push_example(RawExample::new());
    }

    #[test]
    fn extract_from_multi_example() {
        let mut multi_ex = RawMultiExample::new();
        multi_ex.push_example(RawExample::new());
        multi_ex.push_example(RawExample::new());
        multi_ex.push_example(RawExample::new());
        let _ = multi_ex.extract_at(0).unwrap();
        let res = multi_ex.extract_at(7);
        assert!(res.is_none());
    }
}
