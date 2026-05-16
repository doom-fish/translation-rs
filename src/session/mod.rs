use core::ffi::{c_char, c_void};
use core::ptr;

use serde::{Deserialize, Serialize};

use crate::error::TranslationError;
use crate::ffi;
use crate::private::{error_from_status, json_cstring, parse_json_ptr, to_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslationSessionConfiguration {
    source: String,
    target: String,
}

impl TranslationSessionConfiguration {
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationRequest {
    source_text: String,
    client_identifier: Option<String>,
}

impl TranslationRequest {
    #[must_use]
    pub fn new(source_text: impl Into<String>) -> Self {
        Self {
            source_text: source_text.into(),
            client_identifier: None,
        }
    }

    #[must_use]
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    #[must_use]
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }

    #[must_use]
    pub fn with_client_identifier(mut self, client_identifier: impl Into<String>) -> Self {
        self.client_identifier = Some(client_identifier.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationResponse {
    source_language: String,
    target_language: String,
    source_text: String,
    target_text: String,
    client_identifier: Option<String>,
}

impl TranslationResponse {
    #[must_use]
    pub fn source_language(&self) -> &str {
        &self.source_language
    }

    #[must_use]
    pub fn target_language(&self) -> &str {
        &self.target_language
    }

    #[must_use]
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    #[must_use]
    pub fn target_text(&self) -> &str {
        &self.target_text
    }

    #[must_use]
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }
}

pub struct TranslationSession {
    token: *mut c_void,
    configuration: TranslationSessionConfiguration,
}

impl Drop for TranslationSession {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::trl_session_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl TranslationSession {
    pub fn new(configuration: TranslationSessionConfiguration) -> Result<Self, TranslationError> {
        let configuration_json = json_cstring(&configuration)?;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let token = unsafe { ffi::trl_session_new(configuration_json.as_ptr(), &mut err_msg) };
        if token.is_null() {
            return Err(unsafe {
                error_from_status(ffi::status::UNAVAILABLE_ON_THIS_MACOS, err_msg)
            });
        }
        Ok(Self {
            token,
            configuration,
        })
    }

    #[must_use]
    pub fn configuration(&self) -> &TranslationSessionConfiguration {
        &self.configuration
    }

    pub fn prepare_translation(&self) -> Result<(), TranslationError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::trl_session_prepare_translation(self.token, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn translate(&self, text: &str) -> Result<TranslationResponse, TranslationError> {
        let text = to_cstring(text)?;
        let mut response_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_translate_text_json(
                self.token,
                text.as_ptr(),
                &mut response_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(response_json, "translation response") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn translate_batch(
        &self,
        requests: &[TranslationRequest],
    ) -> Result<Vec<TranslationResponse>, TranslationError> {
        let requests_json = json_cstring(requests)?;
        let mut responses_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_translate_batch_json(
                self.token,
                requests_json.as_ptr(),
                &mut responses_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(responses_json, "translation batch responses") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
