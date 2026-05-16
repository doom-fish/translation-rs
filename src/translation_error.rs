use std::error::Error;
use std::fmt;

use crate::ffi;

const FAILURE_REASON_DELIMITER: &str = "\x1f";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationError {
    InvalidArgument(String),
    UnavailableOnThisMacOS(String),
    TimedOut(String),
    UnsupportedSourceLanguage(String),
    UnsupportedTargetLanguage(String),
    UnsupportedLanguagePairing(String),
    UnableToIdentifyLanguage(String),
    NothingToTranslate(String),
    AlreadyCancelled(String),
    NotInstalled(String),
    Framework(String),
    Unknown(String),
}

impl TranslationError {
    #[must_use]
    pub fn error_description(&self) -> &str {
        split_payload(self.message()).0
    }

    #[must_use]
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
