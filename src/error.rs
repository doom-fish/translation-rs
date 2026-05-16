use std::error::Error;
use std::fmt;

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

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            | Self::Unknown(message) => f.write_str(message),
        }
    }
}

impl Error for TranslationError {}
