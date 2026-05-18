use std::error::Error;
use std::fmt;

use crate::ffi;

const FAILURE_REASON_DELIMITER: &str = "";

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enumerates failures reported by Translation.framework and the Swift bridge.
pub enum TranslationError {
    /// An argument failed validation before reaching the framework.
    InvalidArgument(String),
    /// Translation.framework is unavailable on the current macOS version.
    UnavailableOnThisMacOS(String),
    /// The framework operation timed out.
    TimedOut(String),
    /// The source language is unsupported by Translation.framework.
    UnsupportedSourceLanguage(String),
    /// The target language is unsupported by Translation.framework.
    UnsupportedTargetLanguage(String),
    /// The source and target pairing is unsupported by Translation.framework.
    UnsupportedLanguagePairing(String),
    /// The framework could not identify the input language.
    UnableToIdentifyLanguage(String),
    /// The request contained no translatable content.
    NothingToTranslate(String),
    /// The session was already cancelled.
    AlreadyCancelled(String),
    /// Required translation resources are not installed.
    NotInstalled(String),
    /// The Swift bridge or Translation.framework returned an underlying error.
    Framework(String),
    /// An unknown status or payload was returned by the bridge.
    Unknown(String),
}

impl TranslationError {
    #[must_use]
    /// Returns the primary error description from the framework payload.
    pub fn error_description(&self) -> &str {
        split_payload(self.message()).0
    }

    #[must_use]
    /// Returns the framework failure reason, when one was provided.
    pub fn failure_reason(&self) -> Option<&str> {
        split_payload(self.message()).1
    }

    fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::UnavailableOnThisMacOS(message)
            | Self::TimedOut(message)
            | Self::UnsupportedSourceLanguage(message)
            | Self::UnsupportedTargetLanguage(message)
            | Self::UnsupportedLanguagePairing(message)
            | Self::UnableToIdentifyLanguage(message)
            | Self::NothingToTranslate(message)
            | Self::AlreadyCancelled(message)
            | Self::NotInstalled(message)
            | Self::Framework(message)
            | Self::Unknown(message) => message,
        }
    }

    pub(crate) fn from_status_parts(
        status: i32,
        description: String,
        failure_reason: Option<String>,
    ) -> Self {
        let message = encode_payload(description, failure_reason);
        match status {
            ffi::status::INVALID_ARGUMENT => Self::InvalidArgument(message),
            ffi::status::UNAVAILABLE_ON_THIS_MACOS => Self::UnavailableOnThisMacOS(message),
            ffi::status::TIMED_OUT => Self::TimedOut(message),
            ffi::status::UNSUPPORTED_SOURCE_LANGUAGE => Self::UnsupportedSourceLanguage(message),
            ffi::status::UNSUPPORTED_TARGET_LANGUAGE => Self::UnsupportedTargetLanguage(message),
            ffi::status::UNSUPPORTED_LANGUAGE_PAIRING => Self::UnsupportedLanguagePairing(message),
            ffi::status::UNABLE_TO_IDENTIFY_LANGUAGE => Self::UnableToIdentifyLanguage(message),
            ffi::status::NOTHING_TO_TRANSLATE => Self::NothingToTranslate(message),
            ffi::status::ALREADY_CANCELLED => Self::AlreadyCancelled(message),
            ffi::status::NOT_INSTALLED => Self::NotInstalled(message),
            ffi::status::FRAMEWORK_ERROR => Self::Framework(message),
            _ => Self::Unknown(message),
        }
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.failure_reason() {
            Some(failure_reason) => {
                write!(f, "{}: {failure_reason}", self.error_description())
            }
            None => f.write_str(self.error_description()),
        }
    }
}

impl Error for TranslationError {}

fn encode_payload(description: String, failure_reason: Option<String>) -> String {
    match failure_reason {
        Some(failure_reason) if !failure_reason.is_empty() => {
            format!("{description}{FAILURE_REASON_DELIMITER}{failure_reason}")
        }
        _ => description,
    }
}

fn split_payload(message: &str) -> (&str, Option<&str>) {
    match message.split_once(FAILURE_REASON_DELIMITER) {
        Some((description, failure_reason)) if !failure_reason.is_empty() => {
            (description, Some(failure_reason))
        }
        _ => (message, None),
    }
}
