use translation::{
    Language, TranslationError, TranslationRequest, TranslationSession,
    TranslationSessionConfiguration,
};

fn main() -> Result<(), TranslationError> {
    exposes_configuration_properties_and_request_mutation()?;
    streaming_batch_translation_smoke_test()
}

fn exposes_configuration_properties_and_request_mutation() -> Result<(), TranslationError> {
    let configuration = TranslationSessionConfiguration::new("en", "es");
    let session = TranslationSession::new(configuration.clone())?;

    assert_eq!(session.configuration(), &configuration);
    assert!(session
        .source_language()
        .as_ref()
        .map(Language::identifier)
        .is_some_and(|tag| tag.starts_with("en")));
    assert!(session
        .target_language()
        .as_ref()
        .map(Language::identifier)
        .is_some_and(|tag| tag.starts_with("es")));

    let mut request = TranslationRequest::new("hello world");
    request.set_source_text("hola mundo");
    request.set_client_identifier("greeting");
    assert_eq!(request.source_text(), "hola mundo");
    assert_eq!(request.client_identifier(), Some("greeting"));
    request.clear_client_identifier();
    assert_eq!(request.client_identifier(), None);

    match session.can_request_downloads() {
        Ok(_) => {}
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    }

    match session.is_ready() {
        Ok(_) => {}
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    }

    let no_target_session = TranslationSession::new(
        TranslationSessionConfiguration::with_optional_target("en", None),
    )?;
    assert_eq!(no_target_session.target_language(), None);

    Ok(())
}

fn streaming_batch_translation_smoke_test() -> Result<(), TranslationError> {
    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;
    let requests = [TranslationRequest::new("hello world")];

    let mut stream = match session.translate_batch_streaming(&requests) {
        Ok(stream) => stream,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    };

    match stream.try_next() {
        Ok(Some(response)) => assert_eq!(response.source_text(), "hello world"),
        Ok(None)
        | Err(
            TranslationError::NotInstalled(_)
            | TranslationError::Framework(_)
            | TranslationError::UnsupportedLanguagePairing(_)
            | TranslationError::UnsupportedTargetLanguage(_)
            | TranslationError::UnsupportedSourceLanguage(_),
        ) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}
