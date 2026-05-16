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
mod private;
pub mod session;

pub use availability::{LanguageAvailability, LanguageAvailabilityStatus};
pub use detection::detect_language;
pub use error::TranslationError;
pub use session::{
    TranslationRequest, TranslationResponse, TranslationSession, TranslationSessionConfiguration,
};

pub mod prelude {
    pub use crate::availability::{LanguageAvailability, LanguageAvailabilityStatus};
    pub use crate::detection::detect_language;
    pub use crate::error::TranslationError;
    pub use crate::session::{
        TranslationRequest, TranslationResponse, TranslationSession,
        TranslationSessionConfiguration,
    };
}
