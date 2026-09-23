use core::ffi::c_char;
use core::ptr;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use crate::ffi;
use crate::private::{error_from_status, string_from_ptr, to_cstring};
use crate::translation_error::TranslationError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
/// Wraps a Translation.framework language identifier.
pub struct Language {
    identifier: String,
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}

impl Language {
    #[must_use]
    /// Creates a language wrapper from a BCP-47 identifier.
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }

    #[must_use]
    /// Returns the wrapped language identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the Translation.framework-canonicalized language identifier.
    pub fn canonicalized(&self) -> Result<Self, TranslationError> {
        let identifier = to_cstring(self.identifier())?;
        let mut canonicalized: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_language_canonicalize(
                identifier.as_ptr(),
                &raw mut canonicalized,
                &raw mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { string_from_ptr(canonicalized, "canonicalized language") }.map(Self::from)
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        self.identifier()
    }
}

impl From<&str> for Language {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

impl From<String> for Language {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<Language> for String {
    fn from(language: Language) -> Self {
        language.identifier
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.identifier())
    }
}
