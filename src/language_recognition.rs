use core::ffi::c_char;
use core::ptr;

use crate::ffi;
use crate::language::Language;
use crate::private::{error_from_status, string_from_ptr, to_cstring};
use crate::translation_error::TranslationError;

/// Detects the dominant language as a `Language` using `NLLanguageRecognizer`.
pub fn recognize_language(text: &str) -> Result<Option<Language>, TranslationError> {
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
    unsafe { string_from_ptr(language, "detected language") }
        .map(Language::from)
        .map(Some)
}

/// Detects the dominant language identifier using `NLLanguageRecognizer`.
pub fn detect_language(text: &str) -> Result<Option<String>, TranslationError> {
    recognize_language(text).map(|language| language.map(String::from))
}
