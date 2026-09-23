use core::ffi::{c_char, CStr};
use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::ffi;
use crate::translation_error::TranslationError;

#[derive(Debug, Deserialize)]
struct BridgeErrorPayload {
    description: String,
    #[serde(default, rename = "failureReason", alias = "failure_reason")]
    failure_reason: Option<String>,
}

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

/// Convert a nullable C string pointer from the Swift bridge into a Rust String.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is either null or points to a valid, null-terminated UTF-8 C string
/// - `ptr` was allocated by the Swift bridge and must be freed via `trl_string_free`
/// - `ptr` is not accessed after this function returns (the string is moved to a new Rust String)
pub unsafe fn string_from_ptr(ptr: *mut c_char, context: &str) -> Result<String, TranslationError> {
    take_optional_string(ptr).ok_or_else(|| {
        TranslationError::Unknown(format!("missing {context} response from Swift bridge"))
    })
}

/// Parse a JSON string from a C pointer returned by the Swift bridge.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` is either null or points to a valid, null-terminated UTF-8 C string containing valid JSON
/// - `ptr` was allocated by the Swift bridge and must be freed via `trl_string_free`
/// - `ptr` is not accessed after this function returns
pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, TranslationError> {
    let json = string_from_ptr(ptr, context)?;
    serde_json::from_str(&json).map_err(|error| {
        TranslationError::Unknown(format!("failed to decode {context} JSON payload: {error}"))
    })
}

/// Convert a Swift bridge status code into a TranslationError.
///
/// # Safety
///
/// The caller must ensure that:
/// - `err_msg` is either null or points to a valid, null-terminated UTF-8 C string
/// - `err_msg` was allocated by the Swift bridge and must be freed via `trl_string_free`
/// - `err_msg` is not accessed after this function returns
pub unsafe fn error_from_status(status: i32, err_msg: *mut c_char) -> TranslationError {
    let payload = take_optional_string(err_msg)
        .unwrap_or_else(|| format!("Swift bridge call failed with status code {status}"));
    error_from_payload(status, payload)
}

pub fn error_from_payload(status: i32, payload: String) -> TranslationError {
    if let Ok(parsed) = serde_json::from_str::<BridgeErrorPayload>(&payload) {
        return TranslationError::from_status_parts(
            status,
            parsed.description,
            parsed.failure_reason,
        );
    }

    TranslationError::from_status_parts(status, payload, None)
}
