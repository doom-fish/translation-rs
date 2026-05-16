use core::ffi::{c_char, c_void};
use core::ptr;

use crate::ffi;
use crate::language::Language;
use crate::language_pair::LanguagePair;
use crate::private::{error_from_status, parse_json_ptr, to_cstring};
use crate::translation_error::TranslationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageAvailabilityStatus {
    Installed,
    Supported,
    Unsupported,
    Unknown(i32),
}

impl LanguageAvailabilityStatus {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Installed,
            1 => Self::Supported,
            2 => Self::Unsupported,
            other => Self::Unknown(other),
        }
    }
}

pub struct LanguageAvailability {
    token: *mut c_void,
}

impl Drop for LanguageAvailability {
    fn drop(&mut self) {
        if !self.token.is_null() {
            unsafe { ffi::trl_language_availability_release(self.token) };
            self.token = ptr::null_mut();
        }
    }
}

impl LanguageAvailability {
    pub fn new() -> Result<Self, TranslationError> {
        let token = unsafe { ffi::trl_language_availability_new() };
        if token.is_null() {
            return Err(TranslationError::UnavailableOnThisMacOS(
                "LanguageAvailability requires macOS 15+".to_owned(),
            ));
        }
        Ok(Self { token })
    }

    pub fn supported_languages(&self) -> Result<Vec<String>, TranslationError> {
        self.supported_language_objects()
            .map(|languages| languages.into_iter().map(String::from).collect())
    }

    pub fn supported_language_objects(&self) -> Result<Vec<Language>, TranslationError> {
        let mut languages_json: *mut c_char = ptr::null_mut();
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_language_availability_supported_languages_json(
                self.token,
                &mut languages_json,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            unsafe { parse_json_ptr(languages_json, "supported languages") }
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn status_for_pair(
        &self,
        source_language: &str,
        target_language: &str,
    ) -> Result<LanguageAvailabilityStatus, TranslationError> {
        let source_language = Language::from(source_language);
        let target_language = Language::from(target_language);
        self.status_for_languages(&source_language, Some(&target_language))
    }

    pub fn status_for_language_pair(
        &self,
        pair: &LanguagePair,
    ) -> Result<LanguageAvailabilityStatus, TranslationError> {
        self.status_for_languages(pair.source(), pair.target())
    }

    pub fn status_for_languages(
        &self,
        source_language: &Language,
        target_language: Option<&Language>,
    ) -> Result<LanguageAvailabilityStatus, TranslationError> {
        let source_language = to_cstring(source_language.identifier())?;
        let target_language = target_language
            .map(|language| to_cstring(language.identifier()))
            .transpose()?;
        let mut status_raw = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_language_availability_status_from_to(
                self.token,
                source_language.as_ptr(),
                target_language
                    .as_ref()
                    .map_or(ptr::null(), |language| language.as_ptr()),
                &mut status_raw,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(LanguageAvailabilityStatus::from_raw(status_raw))
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }

    pub fn status_for_text(
        &self,
        text: &str,
        target_language: &str,
    ) -> Result<LanguageAvailabilityStatus, TranslationError> {
        let target_language = Language::from(target_language);
        self.status_for_text_in_language(text, Some(&target_language))
    }

    pub fn status_for_text_in_language(
        &self,
        text: &str,
        target_language: Option<&Language>,
    ) -> Result<LanguageAvailabilityStatus, TranslationError> {
        let text = to_cstring(text)?;
        let target_language = target_language
            .map(|language| to_cstring(language.identifier()))
            .transpose()?;
        let mut status_raw = 0;
        let mut err_msg: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::trl_language_availability_status_for_text(
                self.token,
                text.as_ptr(),
                target_language
                    .as_ref()
                    .map_or(ptr::null(), |language| language.as_ptr()),
                &mut status_raw,
                &mut err_msg,
            )
        };
        if status == ffi::status::OK {
            Ok(LanguageAvailabilityStatus::from_raw(status_raw))
        } else {
            Err(unsafe { error_from_status(status, err_msg) })
        }
    }
}
