pub struct Example {
    pub(crate) example: *mut vowpalwabbit_sys::VWExample,
}

impl Example {
    pub(crate) fn get_ptr(&self) -> *const vowpalwabbit_sys::VWExample {
        self.example
    }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWExample {
        self.example
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWExampleDelete(self.get_mut_ptr());
        }
    }
}
