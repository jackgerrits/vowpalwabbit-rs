pub struct Example {
    pub(crate) example: *mut vowpalwabbit_sys::VWExample,
}

impl Example {
    pub fn new() -> Example {
        unsafe {
            Example {
                example: vowpalwabbit_sys::VWExampleCreate(),
            }
        }
    }

    fn get_ptr(&self) -> *const vowpalwabbit_sys::VWExample {
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

#[cfg(test)]
mod tests {
    use super::Example;

    #[test]
    fn create_example() {
        let _ = Example::new();
    }
}
