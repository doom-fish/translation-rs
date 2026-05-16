use core::ffi::c_char;
use core::ptr;

use crate::error::TranslationError;
use crate::ffi;
use crate::private::{error_from_status, string_from_ptr, to_cstring};

pub fn detect_language(text: &str) -> Result<Option<String>, TranslationError> {
    let text = to_cstring(text)?;
    let mut language: *mut c_char = ptr::null_mut();
    let mut err_msg: *mut c_char = ptr::null_mut();
    let status = unsafe { ffi::trl_detect_language(text.as_ptr(), &mut language, &mut err_msg) };
    if status != ffi::status::OK {
        return Err(unsafe { error_from_status(status, err_msg) });
    }
    if language.is_null() {
        return Ok(None);
    }
    unsafe { string_from_ptr(language, "detected language") }.map(Some)
}
