#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

#[cfg(feature = "async")]
/// Async wrappers for Translation.framework operations.
pub mod async_api;
/// Availability APIs mirroring Translation.framework's `LanguageAvailability`.
pub mod availability;
/// Language recognition helpers used alongside Translation.framework.
pub mod detection;
/// Error re-exports for Translation.framework operations.
pub mod error;
pub mod ffi;
/// Language identifier types used by Translation.framework.
pub mod language;
/// `LanguageAvailability` bindings for Translation.framework.
pub mod language_availability;
/// Source/target language pair types for Translation.framework.
pub mod language_pair;
/// NaturalLanguage-backed detection helpers for Translation workflows.
pub mod language_recognition;
mod private;
/// Session re-exports for Translation.framework workflows.
pub mod session;
/// Mutable configuration helpers for Translation.framework sessions.
pub mod translation_configuration;
/// Error types for Translation.framework operations.
pub mod translation_error;
/// Response types returned by Translation.framework translations.
pub mod translation_response;
/// Core `TranslationSession` wrappers for Translation.framework.
pub mod translation_session;

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

/// Common Translation.framework wrapper types for glob imports.
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
