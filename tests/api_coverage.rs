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
        .with_target("es")
        .with_preferred_strategy(TranslationStrategy::LowLatency);
    let _session_configuration = TranslationSessionConfiguration::new("en", "es")
        .with_preferred_strategy(TranslationStrategy::HighFidelity);
    let attributed = TranslationAttributedString::new("hello world")
        .with_skip_translation_for_substring("world")?;
    let _request = TranslationRequest::new("hello world")
        .with_attributed_source_text(attributed);
    let _response = TranslationResponse::new("en", "es", "hello world", "hola mundo")
        .with_attributed_source_text("hello world")
        .with_attributed_target_text("hola mundo");
    let _: LanguageAvailabilityStatus = LanguageAvailabilityStatus::Supported;
    let _: SkipTranslationAttribute = TranslationAttributes::skips_translation(true);

    let _ = std::mem::size_of::<availability::LanguageAvailabilityStatus>();
    let _ = std::mem::size_of::<error::TranslationError>();
    let _ = std::mem::size_of::<session::TranslationRequest>();
    let _ = std::mem::size_of::<session::TranslationAttributedString>();
    let _ = TranslationSession::translate_attributed;
    let _ = LanguageAvailability::with_preferred_strategy;

    let detected = detection::detect_language("hello world")?;
    assert!(detected.is_some_and(|language| language.starts_with("en")));
    Ok(())
}
