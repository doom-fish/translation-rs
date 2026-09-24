use core::ffi::{c_char, c_void};
use core::ptr;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::language::Language;
use crate::language_pair::LanguagePair;
use crate::private::{error_from_status, json_cstring, parse_json_ptr, to_cstring};
use crate::translation_attributes::TranslationAttributedString;
use crate::translation_configuration::TranslationConfiguration;
use crate::translation_error::TranslationError;
use crate::translation_response::TranslationResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Mirrors `TranslationSession.Strategy` from Translation.framework.
pub enum TranslationStrategy {
    /// Prefer higher-quality translations when available.
    #[default]
    HighFidelity,
    /// Prefer lower-latency translations when available.
    LowLatency,
}

/// Alias mirroring `TranslationSession.Strategy` inside the `translation_session` module.
pub use TranslationStrategy as Strategy;

impl TranslationStrategy {
    pub(crate) const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::HighFidelity),
            1 => Some(Self::LowLatency),
            _ => None,
        }
    }

    pub(crate) const fn raw(self) -> i32 {
        match self {
            Self::HighFidelity => 0,
            Self::LowLatency => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Serializable mirror of `TranslationSession.Configuration` from Translation.framework.
pub struct TranslationSessionConfiguration {
    source: String,
    target: Option<String>,
    #[serde(default)]
    preferred_strategy: TranslationStrategy,
}

impl TranslationSessionConfiguration {
    #[must_use]
    /// Creates a session configuration from explicit source and target identifiers.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: Some(target.into()),
            preferred_strategy: TranslationStrategy::default(),
        }
    }

    #[must_use]
    /// Creates a session configuration with an optional target identifier.
    pub fn with_optional_target(source: impl Into<String>, target: Option<String>) -> Self {
        Self {
            source: source.into(),
            target,
            preferred_strategy: TranslationStrategy::default(),
        }
    }

    #[must_use]
    /// Creates a session configuration from a `LanguagePair`.
    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Self {
        let pair = pair.into();
        Self {
            source: pair.source().identifier().to_owned(),
            target: pair
                .target()
                .map(|language| language.identifier().to_owned()),
            preferred_strategy: TranslationStrategy::default(),
        }
    }

    /// Converts a `TranslationConfiguration` into a session configuration.
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
        )
        .with_preferred_strategy(configuration.preferred_strategy()))
    }

    #[must_use]
    /// Returns the source language identifier.
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    /// Returns the target identifier or an empty string when unspecified.
    pub fn target(&self) -> &str {
        self.target.as_deref().unwrap_or("")
    }

    #[must_use]
    /// Returns the optional target language identifier.
    pub fn optional_target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    #[must_use]
    /// Returns the preferred Translation.framework strategy.
    pub const fn preferred_strategy(&self) -> TranslationStrategy {
        self.preferred_strategy
    }

    #[must_use]
    /// Returns a copy with the given preferred strategy.
    pub fn with_preferred_strategy(mut self, preferred_strategy: TranslationStrategy) -> Self {
        self.preferred_strategy = preferred_strategy;
        self
    }

    #[must_use]
    /// Returns this configuration as a `LanguagePair`.
    pub fn language_pair(&self) -> LanguagePair {
        LanguagePair::new(
            Language::from(self.source.clone()),
            self.target.clone().map(Language::from),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Serializable counterpart to `TranslationSession.Request` in Translation.framework.
pub struct TranslationRequest {
    source_text: String,
    #[serde(default)]
    attributed_source_text: Option<TranslationAttributedString>,
    client_identifier: Option<String>,
}

impl TranslationRequest {
    #[must_use]
    /// Creates a translation request from source text.
    pub fn new(source_text: impl Into<String>) -> Self {
        Self {
            source_text: source_text.into(),
            attributed_source_text: None,
            client_identifier: None,
        }
    }

    #[must_use]
    /// Creates a translation request from attributed source text.
    pub fn from_attributed_source_text(
        source_text: impl Into<TranslationAttributedString>,
    ) -> Self {
        let source_text = source_text.into();
        Self {
            source_text: source_text.text().to_owned(),
            attributed_source_text: Some(source_text),
            client_identifier: None,
        }
    }

    #[must_use]
    /// Returns the source text to translate.
    pub fn source_text(&self) -> &str {
        &self.source_text
    }

    #[must_use]
    /// Returns the attributed source text when this request carries attributed input.
    pub fn attributed_source_text(&self) -> Option<&TranslationAttributedString> {
        self.attributed_source_text.as_ref()
    }

    /// Replaces the source text to translate and clears attributed input.
    pub fn set_source_text(&mut self, source_text: impl Into<String>) {
        self.source_text = source_text.into();
        self.attributed_source_text = None;
    }

    /// Replaces the attributed source text to translate.
    pub fn set_attributed_source_text(
        &mut self,
        attributed_source_text: impl Into<TranslationAttributedString>,
    ) {
        let attributed_source_text = attributed_source_text.into();
        attributed_source_text.text().clone_into(&mut self.source_text);
        self.attributed_source_text = Some(attributed_source_text);
    }

    /// Clears the attributed source text payload while preserving plain text.
    pub fn clear_attributed_source_text(&mut self) {
        self.attributed_source_text = None;
    }

    #[must_use]
    /// Returns the client identifier, if one is set.
    pub fn client_identifier(&self) -> Option<&str> {
        self.client_identifier.as_deref()
    }

    /// Sets the client identifier for correlation with framework responses.
    pub fn set_client_identifier(&mut self, client_identifier: impl Into<String>) {
        self.client_identifier = Some(client_identifier.into());
    }

    /// Clears the client identifier.
    pub fn clear_client_identifier(&mut self) {
        self.client_identifier = None;
    }

    #[must_use]
    /// Returns a copy with the given attributed source text.
    pub fn with_attributed_source_text(
        mut self,
        attributed_source_text: impl Into<TranslationAttributedString>,
    ) -> Self {
        self.set_attributed_source_text(attributed_source_text);
        self
    }

    #[must_use]
    /// Returns a copy with the given client identifier.
    pub fn with_client_identifier(mut self, client_identifier: impl Into<String>) -> Self {
        self.set_client_identifier(client_identifier);
        self
    }
}

/// Streams responses from `TranslationSession.translations(from:)`.
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
    /// Advances to the next streaming translation response.
    pub fn try_next(&mut self) -> Result<Option<TranslationResponse>, TranslationError> {
        if self.finished {
            return Ok(None);
        }

        let mut response_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_batch_response_next_json(self.token, &raw mut response_json, &raw mut err_msg)
        };
        if status != ffi::status::OK {
            let error = unsafe { error_from_status(status, err_msg) };
            if !matches!(
                error,
                TranslationError::TimedOut(_) | TranslationError::MainRunLoopNotRunning(_)
            ) {
                self.finished = true;
            }
            return Err(error);
        }
        if response_json.is_null() {
            self.finished = true;
            return Ok(None);
        }
        unsafe { parse_json_ptr(response_json, "streaming translation response") }.map(Some)
    }

    /// Collects all remaining streaming responses into a vector.
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
        self.try_next().transpose()
    }
}

/// Wraps Translation.framework's `TranslationSession`.
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
    /// Creates a session from a `TranslationSessionConfiguration`.
    pub fn new(configuration: TranslationSessionConfiguration) -> Result<Self, TranslationError> {
        let configuration_json = json_cstring(&configuration)?;
        let mut token: *mut c_void = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_new(
                configuration_json.as_ptr(),
                &raw mut token,
                &raw mut err_msg,
            )
        };
        if status == ffi::status::OK && !token.is_null() {
            Ok(Self {
                token,
                configuration,
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Creates a session from a `LanguagePair`.
    pub fn from_language_pair(pair: impl Into<LanguagePair>) -> Result<Self, TranslationError> {
        Self::new(TranslationSessionConfiguration::from_language_pair(pair))
    }

    /// Creates a session from a mutable `TranslationConfiguration`.
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
    /// Returns the session configuration.
    pub fn configuration(&self) -> &TranslationSessionConfiguration {
        &self.configuration
    }

    #[must_use]
    /// Returns the configured source language.
    pub fn source_language(&self) -> Option<Language> {
        Some(Language::from(self.configuration.source().to_owned()))
    }

    #[must_use]
    /// Returns the configured target language, if one was set.
    pub fn target_language(&self) -> Option<Language> {
        self.configuration
            .optional_target()
            .map(|language| Language::from(language.to_owned()))
    }

    /// Returns the Translation.framework preferred strategy for this session.
    pub fn preferred_strategy(&self) -> Result<TranslationStrategy, TranslationError> {
        let mut raw = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_preferred_strategy(self.token, &raw mut raw, &raw mut err_msg)
        };
        if status == ffi::status::OK {
            TranslationStrategy::from_raw(raw).ok_or_else(|| {
                TranslationError::Unknown(format!(
                    "unknown TranslationSession.Strategy raw value returned by Swift bridge: {raw}"
                ))
            })
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Reports whether the session can request language pack downloads.
    pub fn can_request_downloads(&self) -> Result<bool, TranslationError> {
        self.read_bool(ffi::trl_session_can_request_downloads)
    }

    /// Reports whether Translation.framework considers the session ready.
    pub fn is_ready(&self) -> Result<bool, TranslationError> {
        self.read_bool(ffi::trl_session_is_ready)
    }

    /// Cancels the underlying Translation.framework session.
    pub fn cancel(&self) -> Result<(), TranslationError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::trl_session_cancel(self.token, &raw mut err_msg) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Prepares language resources via `TranslationSession.prepareTranslation()`.
    pub fn prepare_translation(&self) -> Result<(), TranslationError> {
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::trl_session_prepare_translation(self.token, &raw mut err_msg) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Translates a single string with `TranslationSession.translate(_:)`.
    pub fn translate(&self, text: &str) -> Result<TranslationResponse, TranslationError> {
        let text = to_cstring(text)?;
        let mut response_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_translate_text_json(
                self.token,
                text.as_ptr(),
                &raw mut response_json,
                &raw mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(response_json, "translation response") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Translates attributed text with `TranslationSession.translate(_:)`.
    pub fn translate_attributed(
        &self,
        text: &TranslationAttributedString,
    ) -> Result<TranslationResponse, TranslationError> {
        let text_json = json_cstring(text)?;
        let mut response_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_session_translate_attributed_json(
                self.token,
                text_json.as_ptr(),
                &raw mut response_json,
                &raw mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(response_json, "translation response") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Translates a batch with `TranslationSession.translations(from:)`.
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
                &raw mut responses_json,
                &raw mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(responses_json, "translation batch responses") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    /// Starts a streaming batch translation for `TranslationSession.translations(from:)`.
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
                &raw mut batch_token,
                &raw mut err_msg,
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
        let status = unsafe { ffi_fn(self.token, &raw mut value, &raw mut err_msg) };
        if status == ffi::status::OK {
            Ok(value != 0)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
