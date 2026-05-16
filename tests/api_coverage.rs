use translation::prelude::*;
use translation::{availability, detection, error, session};

fn main() -> Result<(), TranslationError> {
    prelude_and_compatibility_modules_expose_surface()
}

fn prelude_and_compatibility_modules_expose_surface() -> Result<(), TranslationError> {
    let _language = Language::new("en");
    let _pair = LanguagePair::between("en", "es");
    let _configuration = TranslationConfiguration::new()
        .with_source("en")
        .with_target("es");
    let _session_configuration = TranslationSessionConfiguration::new("en", "es");
    let _request = TranslationRequest::new("hello world");
    let _response = TranslationResponse::new("en", "es", "hello world", "hola mundo");
    let _: LanguageAvailabilityStatus = LanguageAvailabilityStatus::Supported;

    let _ = std::mem::size_of::<availability::LanguageAvailabilityStatus>();
    let _ = std::mem::size_of::<error::TranslationError>();
    let _ = std::mem::size_of::<session::TranslationRequest>();

    let detected = detection::detect_language("hello world")?;
    assert!(detected.is_some_and(|language| language.starts_with("en")));
    Ok(())
}
