use core::ffi::{c_char, c_void};
use core::ptr;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::language::Language;
use crate::language_pair::LanguagePair;
use crate::private::{error_from_status, json_cstring, parse_json_ptr, to_cstring};
use crate::translation_configuration::TranslationConfiguration;
use crate::translation_error::TranslationError;
use crate::translation_response::TranslationResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslationSessionConfiguration {
    source: String,
    target: Option<String>,
}

impl TranslationSessionConfiguration {
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: Some(target.into()),
        }
    }

    #[must_use]
    pub fn with_optional_target(source: impl Into<String>, target: Option<String>) -> Self {
        Self {
            source: source.into(),
            target,
        }
    }

    #[must_use]
    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Self {
        let pair = pair.into();
        Self {
            source: pair.source().identifier().to_owned(),
            target: pair
                .target()
                .map(|language| language.identifier().to_owned()),
        }
    }

    pub fn try_from_translation_configuration(
        configuration: &TranslationConfiguration,
    ) -> Result<Self, TranslationError> {
        let source = configuration.source_identifier().ok_or_else(|| {
            TranslationError::InvalidArgument(
                "manual TranslationSession construction requires a source language".to_owned(),
            )
        })?;
        Ok(Self::with_optional_target(
            source.to_owned(),
            configuration.target_identifier().map(ToOwned::to_owned),
        ))
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn target(&self) -> &str {
        self.target.as_deref().unwrap_or("")
    }

    #[must_use]
    pub fn optional_target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    #[must_use]
    pub fn language_pair(&self) -> LanguagePair {
        LanguagePair::new(
            Language::from(self.source.clone()),
            self.target.clone().map(Language::from),
        )
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

    pub fn set_source_text(&mut self, source_text: impl Into<String>) {
        self.source_text = source_text.into();
    }

    #[must_use]
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }

    pub fn set_client_identifier(&mut self, client_identifier: impl Into<String>) {
        self.client_identifier = Some(client_identifier.into());
    }

    pub fn clear_client_identifier(&mut self) {
        self.client_identifier = None;
    }

    #[must_use]
    pub fn with_client_identifier(mut self, client_identifier: impl Into<String>) -> Self {
        self.set_client_identifier(client_identifier);
        self
    }
}

pub struct TranslationBatchResponse {
    token: *mut c_void,
    finished: bool,
}

impl Drop for TranslationBatchResponse {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::trl_batch_response_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl TranslationBatchResponse {
    pub fn try_next(&mut self) -> Result<Option<TranslationResponse>, TranslationError> {
        if self.finished {
            return Ok(None);
        }

        let mut response_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_batch_response_next_json(self.token, &mut response_json, &mut err_msg)
        };
        if status != ffi::status::OK {
            self.finished = true;
            return Err(unsafe { error_from_status(status, err_msg) });
        }
        if response_json.is_null() {
            self.finished = true;
            return Ok(None);
        }
        unsafe { parse_json_ptr(response_json, "streaming translation response") }.map(Some)
    }

    pub fn collect_all(mut self) -> Result<Vec<TranslationResponse>, TranslationError> {
        let mut responses = Vec::new();
        while let Some(response) = self.try_next()? {
            responses.push(response);
        }
        Ok(responses)
    }
}

impl Iterator for TranslationBatchResponse {
    type Item = Result<TranslationResponse, TranslationError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(response)) => Some(Ok(response)),
            Ok(None) => None,
            Err(error) => {
                self.finished = true;
                Some(Err(error))
            }
        }
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
        let mut token: *mut c_void = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status =
            unsafe { ffi::trl_session_new(configuration_json.as_ptr(), &mut token, &mut err_msg) };
        if status == ffi::status::OK && !token.is_null() {
            Ok(Self {
                token,
                configuration,
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Result<Self, TranslationError> {
        Self::new(TranslationSessionConfiguration::from_language_pair(pair))
    }

    pub fn from_translation_configuration(
        configuration: &TranslationConfiguration,
    ) -> Result<Self, TranslationError> {
        Self::new(
            TranslationSessionConfiguration::try_from_translation_configuration(configuration)?,
        )
    }

    #[cfg(feature = "async")]
    pub(crate) const fn raw_token(&self) -> *mut c_void {
        self.token
    }

    #[must_use]
    pub fn configuration(&self) -> &TranslationSessionConfiguration {
        &self.configuration
    }

    #[must_use]
    pub fn source_language(&self) -> Option<Language> {
        Some(Language::from(self.configuration.source().to_owned()))
    }

    #[must_use]
    pub fn target_language(&self) -> Option<Language> {
        self.configuration
            .optional_target()
            .map(|language| Language::from(language.to_owned()))
    }

    pub fn can_request_downloads(&self) -> Result<bool, TranslationError> {
        self.read_bool(ffi::trl_session_can_request_downloads)
    }

    pub fn is_ready(&self) -> Result<bool, TranslationError> {
        self.read_bool(ffi::trl_session_is_ready)
    }

    pub fn cancel(&self) -> Result<(), TranslationError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::trl_session_cancel(self.token, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
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

    pub fn translate_batch_streaming(
        &self,
        requests: &[TranslationRequest],
    ) -> Result<TranslationBatchResponse, TranslationError> {
        let requests_json = json_cstring(requests)?;
        let mut batch_token: *mut c_void = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_translate_batch_stream_json(
                self.token,
                requests_json.as_ptr(),
                &mut batch_token,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK && !batch_token.is_null() {
            Ok(TranslationBatchResponse {
                token: batch_token,
                finished: false,
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    fn read_bool(
        &self,
        ffi_fn: unsafe extern "C" fn(*mut c_void, *mut i32, *mut *mut c_char) -> i32,
    ) -> Result<bool, TranslationError> {
        let mut value = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi_fn(self.token, &mut value, &mut err_msg) };
        if status == ffi::status::OK {
            Ok(value != 0)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
