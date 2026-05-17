#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod availability;
pub mod detection;
pub mod error;
pub mod ffi;
pub mod language;
pub mod language_availability;
pub mod language_pair;
pub mod language_recognition;
mod private;
pub mod session;
pub mod translation_configuration;
pub mod translation_error;
pub mod translation_response;
pub mod translation_session;
#[cfg(feature = "async")]
pub mod async_api;

pub use language::Language;
pub use language_availability::{LanguageAvailability, LanguageAvailabilityStatus};
pub use language_pair::LanguagePair;
pub use language_recognition::{detect_language, recognize_language};
pub use translation_configuration::TranslationConfiguration;
pub use translation_error::TranslationError;
pub use translation_response::TranslationResponse;
pub use translation_session::{
    TranslationBatchResponse, TranslationRequest, TranslationSession,
    TranslationSessionConfiguration,
};

pub mod prelude {
    #[cfg(feature = "async")]
    pub use crate::async_api::{AsyncLanguageAvailability, AsyncTranslationSession};
    pub use crate::language::Language;
    pub use crate::language_availability::{LanguageAvailability, LanguageAvailabilityStatus};
    pub use crate::language_pair::LanguagePair;
    pub use crate::language_recognition::{detect_language, recognize_language};
    pub use crate::translation_configuration::TranslationConfiguration;
    pub use crate::translation_error::TranslationError;
    pub use crate::translation_response::TranslationResponse;
    pub use crate::translation_session::{
        TranslationBatchResponse, TranslationRequest, TranslationSession,
        TranslationSessionConfiguration,
    };
}
