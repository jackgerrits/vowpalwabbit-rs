pub struct Example {
    pub(crate) example: *mut vowpalwabbit_sys::VWExample,
}

impl Drop for Example {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWExampleDelete(self.get_mut_ptr());
        }
    }
}

impl Example {
    // fn get_ptr(&self) -> *const vowpalwabbit_sys::VWExample {
    //     self.example
    // }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWExample {
        self.example
    }

    pub fn clear(mut self) -> RawExample {
        unsafe { vowpalwabbit_sys::VWExampleClear(self.get_mut_ptr()) }
        let new_ex = RawExample {
            example: self.get_mut_ptr(),
        };
        std::mem::forget(self);
        new_ex
    }

    pub(crate) fn release(self) -> *mut vowpalwabbit_sys::VWExample {
        let raw = self.example;
        std::mem::forget(self);
        raw
    }
}

pub struct RawExample {
    pub(crate) example: *mut vowpalwabbit_sys::VWExample,
}

unsafe impl Send for Example {}
unsafe impl Sync for Example {}
unsafe impl Send for RawExample {}
unsafe impl Sync for RawExample {}
impl RawExample {
    pub fn new() -> RawExample {
        unsafe {
            RawExample {
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

    pub(crate) fn release(self) -> *mut vowpalwabbit_sys::VWExample {
        let raw = self.example;
        std::mem::forget(self);
        raw
    }
}

impl Default for RawExample {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RawExample {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWExampleDelete(self.get_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RawExample;

    #[test]
    fn create_example() {
        let _ = RawExample::new();
    }
}
