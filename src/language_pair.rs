use core::ffi::c_char;
use core::ptr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::ffi;
use crate::language::Language;
use crate::private::{error_from_status, parse_json_ptr, to_cstring};
use crate::translation_error::TranslationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct LanguagePair {
    source: Language,
    target: Option<Language>,
}

impl<'de> Deserialize<'de> for LanguagePair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LanguagePairPayload {
            source: Language,
            target: Option<Language>,
        }

        let payload = LanguagePairPayload::deserialize(deserializer)?;
        Ok(Self::new(payload.source, payload.target))
    }
}

impl LanguagePair {
    #[must_use]
    pub fn new(source: impl Into<Language>, target: Option<Language>) -> Self {
        Self {
            source: source.into(),
            target,
        }
    }

    #[must_use]
    pub fn between(source: impl Into<Language>, target: impl Into<Language>) -> Self {
        Self::new(source, Some(target.into()))
    }

    #[must_use]
    pub fn source(&self) -> &Language {
        &self.source
    }

    #[must_use]
    pub fn target(&self) -> Option<&Language> {
        self.target.as_ref()
    }

    pub fn canonicalized(&self) -> Result<Self, TranslationError> {
        let source = to_cstring(self.source.identifier())?;
        let target = self
            .target
            .as_ref()
            .map(|language| to_cstring(language.identifier()))
            .transpose()?;
        let mut pair_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_language_pair_canonicalize_json(
                source.as_ptr(),
                target.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                &mut pair_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(pair_json, "canonicalized language pair") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
