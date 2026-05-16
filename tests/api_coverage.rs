use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let availability = LanguageAvailability::new()?;
    let supported_languages = availability.supported_languages()?;
    assert!(!supported_languages.is_empty());

    let _ = availability.status_for_pair("en", "es")?;
    let _ = availability.status_for_text("hello world", "es")?;
    let detected_language = detect_language("hello world")?;

    let configuration = TranslationSessionConfiguration::new("en", "es");
    let session = TranslationSession::new(configuration.clone())?;
    assert_eq!(session.configuration(), &configuration);

    let request = TranslationRequest::new("hello world").with_client_identifier("hello");
    assert_eq!(request.source_text(), "hello world");
    assert_eq!(request.client_identifier(), Some("hello"));
    assert!(detected_language.is_some());
    Ok(())
}
