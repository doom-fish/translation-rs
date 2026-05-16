use translation::{
    Language, LanguageAvailability, LanguageAvailabilityStatus, LanguagePair, TranslationError,
};

fn main() -> Result<(), TranslationError> {
    supported_languages_and_status_queries_round_trip()
}

fn supported_languages_and_status_queries_round_trip() -> Result<(), TranslationError> {
    let availability = match LanguageAvailability::new() {
        Ok(availability) => availability,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    };

    let languages = availability.supported_language_objects()?;
    assert!(!languages.is_empty());

    let pair = LanguagePair::between("en", "es");
    let pair_status = availability.status_for_language_pair(&pair)?;
    let string_status = availability.status_for_pair("en", "es")?;
    assert_eq!(pair_status, string_status);

    let source = Language::new("en");
    let status_without_target = availability.status_for_languages(&source, None)?;
    assert!(matches!(
        status_without_target,
        LanguageAvailabilityStatus::Installed
            | LanguageAvailabilityStatus::Supported
            | LanguageAvailabilityStatus::Unsupported
            | LanguageAvailabilityStatus::Unknown(_)
    ));

    Ok(())
}
