pub struct Example {
    pub(crate) example: *mut vowpalwabbit_sys::VWExample,
}

unsafe impl Send for Example {}
unsafe impl Sync for Example {}

impl Example {
    pub fn new() -> Example {
        unsafe {
            Example {
                example: vowpalwabbit_sys::VWExampleCreate(),
            }
        }
    }

    // fn get_ptr(&self) -> *const vowpalwabbit_sys::VWExample {
    //     self.example
    // }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWExample {
        self.example
    }

    pub fn clear(&mut self) {
        unsafe { vowpalwabbit_sys::VWExampleClear(self.get_mut_ptr()) }
    }

    pub fn release(self) -> *mut vowpalwabbit_sys::VWExample {
        let raw = self.example;
        std::mem::forget(self);
        raw
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWExampleDelete(self.get_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Example;

    #[test]
    fn create_example() {
        let _ = Example::new();
    }
}
