use core::ffi::{c_char, CStr};
use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::TranslationError;
use crate::ffi;

pub fn to_cstring(value: &str) -> Result<CString, TranslationError> {
    CString::new(value).map_err(|_| {
        TranslationError::InvalidArgument("string contained an interior NUL byte".to_owned())
    })
}

pub fn json_cstring<T: Serialize + ?Sized>(value: &T) -> Result<CString, TranslationError> {
    let json = serde_json::to_string(value).map_err(|error| {
        TranslationError::Unknown(format!("failed to encode JSON payload: {error}"))
    })?;
    to_cstring(&json)
}

pub unsafe fn take_optional_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::trl_string_free(ptr);
    Some(string)
}

pub unsafe fn string_from_ptr(ptr: *mut c_char, context: &str) -> Result<String, TranslationError> {
    take_optional_string(ptr).ok_or_else(|| {
        TranslationError::Unknown(format!("missing {context} response from Swift bridge"))
    })
}

pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, TranslationError> {
    let json = string_from_ptr(ptr, context)?;
    serde_json::from_str(&json).map_err(|error| {
        TranslationError::Unknown(format!("failed to decode {context} JSON payload: {error}"))
    })
}

pub unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> TranslationError {
    let message = take_optional_string(err_msg)
        .unwrap_or_else(|| format!("Swift bridge call failed with status code {status}"));
    match status {
        ffi::status::INVALID_ARGUMENT => TranslationError::InvalidArgument(message),
        ffi::status::UNAVAILABLE_ON_THIS_MACOS => TranslationError::UnavailableOnThisMacOS(message),
        ffi::status::TIMED_OUT => TranslationError::TimedOut(message),
        ffi::status::UNSUPPORTED_SOURCE_LANGUAGE => {
            TranslationError::UnsupportedSourceLanguage(message)
        }
        ffi::status::UNSUPPORTED_TARGET_LANGUAGE => {
            TranslationError::UnsupportedTargetLanguage(message)
        }
        ffi::status::UNSUPPORTED_LANGUAGE_PAIRING => {
            TranslationError::UnsupportedLanguagePairing(message)
        }
        ffi::status::UNABLE_TO_IDENTIFY_LANGUAGE => {
            TranslationError::UnableToIdentifyLanguage(message)
        }
        ffi::status::NOTHING_TO_TRANSLATE => TranslationError::NothingToTranslate(message),
        ffi::status::ALREADY_CANCELLED => TranslationError::AlreadyCancelled(message),
        ffi::status::NOT_INSTALLED => TranslationError::NotInstalled(message),
        ffi::status::FRAMEWORK_ERROR => TranslationError::Framework(message),
        _ => TranslationError::Unknown(message),
    }
}
