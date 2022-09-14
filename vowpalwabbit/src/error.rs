use std::ffi::CStr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VWError {
    #[error("Generic failure")]
    Failure(String),
}

pub(crate) struct ErrorMessageHolder {
    error_message: *mut vowpalwabbit_sys::VWErrorMessage,
}

impl ErrorMessageHolder {
    pub(crate) fn new() -> Self {
        unsafe {
            ErrorMessageHolder {
                error_message: vowpalwabbit_sys::VWErrorMessageCreate(),
            }
        }
    }

    pub(crate) fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWErrorMessage {
        self.error_message
    }

    pub(crate) fn get(&self) -> Option<String> {
        let value = unsafe { vowpalwabbit_sys::VWErrorMessageGetValue(self.error_message) };

        if value.is_null() {
            None
        } else {
            unsafe { Some(CStr::from_ptr(value).to_string_lossy().into_owned()) }
        }
    }

    pub(crate) fn clear(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWErrorMessageClearValue(self.error_message);
        }
    }
}

impl Drop for ErrorMessageHolder {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWErrorMessageDelete(self.error_message);
        }
    }
}

macro_rules! check_result {
    ($a:expr,$b:expr) => {{
        if $a != VW_STATUS_SUCCESS {
            return match $b.get() {
                Some(message) => Err(VWError::Failure(message)),
                None => Err(VWError::Failure("Unknown".to_string())),
            };
        }
    }};
}

pub(crate) use check_result;

#[cfg(test)]
mod tests {
    use super::ErrorMessageHolder;

    #[test]
    fn create_error_message_holder() {
        let _ = ErrorMessageHolder::new();
    }

    #[test]
    fn get_value_error_message_holder() {
        let holder = ErrorMessageHolder::new();
        let maybe_value = holder.get();
        assert!(maybe_value.is_none());
    }

    #[test]
    fn clear_error_message_holder() {
        let mut holder = ErrorMessageHolder::new();
        let maybe_value = holder.get();
        assert!(maybe_value.is_none());
        holder.clear();
        assert!(maybe_value.is_none());
    }
}
