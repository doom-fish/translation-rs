use translation::{
    LanguageAvailability, SkipTranslationAttribute, TranslationAttributedString,
    TranslationAttributes, TranslationConfiguration, TranslationError, TranslationRequest,
    TranslationSession, TranslationSessionConfiguration, TranslationStrategy,
};

fn main() -> Result<(), TranslationError> {
    preferred_strategy_round_trips()?;
    attributed_translation_round_trips()?;
    skip_translation_attribute_round_trips();
    Ok(())
}

fn preferred_strategy_round_trips() -> Result<(), TranslationError> {
    let mut configuration = TranslationConfiguration::new()
        .with_source("en")
        .with_target("fr");
    assert_eq!(configuration.preferred_strategy(), TranslationStrategy::HighFidelity);
    configuration.set_preferred_strategy(TranslationStrategy::LowLatency);
    assert_eq!(configuration.preferred_strategy(), TranslationStrategy::LowLatency);

    let session_configuration = TranslationSessionConfiguration::new("en", "fr")
        .with_preferred_strategy(TranslationStrategy::LowLatency);
    assert_eq!(
        session_configuration.preferred_strategy(),
        TranslationStrategy::LowLatency
    );

    let session = match TranslationSession::new(session_configuration.clone()) {
        Ok(session) => session,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    assert_eq!(session.configuration(), &session_configuration);
    match session.preferred_strategy() {
        Ok(strategy) => assert_eq!(strategy, TranslationStrategy::LowLatency),
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    }

    let availability = match LanguageAvailability::with_preferred_strategy(
        TranslationStrategy::LowLatency,
    ) {
        Ok(availability) => availability,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    assert_eq!(
        availability.preferred_strategy()?,
        TranslationStrategy::LowLatency
    );

    Ok(())
}

fn attributed_translation_round_trips() -> Result<(), TranslationError> {
    let attributed = TranslationAttributedString::new("Translate cargo please")
        .with_skip_translation_for_substring("cargo")?;
    let request = TranslationRequest::new("Translate cargo please")
        .with_client_identifier("attributed")
        .with_attributed_source_text(attributed.clone());
    assert_eq!(request.source_text(), "Translate cargo please");
    assert_eq!(request.attributed_source_text(), Some(&attributed));

    let session = match TranslationSession::new(TranslationSessionConfiguration::new("en", "fr")) {
        Ok(session) => session,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    };

    let batch_responses = match session.translate_batch(&[request]) {
        Ok(responses) => responses,
        Err(
            TranslationError::UnavailableOnThisMacOS(_)
            | TranslationError::NotInstalled(_)
            | TranslationError::Framework(_)
            | TranslationError::UnsupportedLanguagePairing(_)
            | TranslationError::UnsupportedTargetLanguage(_)
            | TranslationError::UnsupportedSourceLanguage(_),
        ) => return Ok(()),
        Err(error) => return Err(error),
    };
    assert_eq!(batch_responses.len(), 1);
    assert_eq!(batch_responses[0].source_text(), "Translate cargo please");
    assert_eq!(batch_responses[0].client_identifier(), Some("attributed"));

    let attributed_response = match session.translate_attributed(&attributed) {
        Ok(response) => response,
        Err(
            TranslationError::UnavailableOnThisMacOS(_)
            | TranslationError::NotInstalled(_)
            | TranslationError::Framework(_)
            | TranslationError::UnsupportedLanguagePairing(_)
            | TranslationError::UnsupportedTargetLanguage(_)
            | TranslationError::UnsupportedSourceLanguage(_),
        ) => return Ok(()),
        Err(error) => return Err(error),
    };
    let attributed_source = attributed_response
        .attributed_source_text()
        .expect("direct attributed translation should preserve source attributes");
    let attributed_target = attributed_response
        .attributed_target_text()
        .expect("direct attributed translation should preserve target attributes");
    assert_eq!(attributed_source.text(), "Translate cargo please");
    assert!(attributed_source
        .skip_translation_runs()
        .iter()
        .any(|run| run.value() == SkipTranslationAttribute::enabled()));
    assert!(attributed_target
        .skip_translation_runs()
        .iter()
        .any(|run| run.value() == SkipTranslationAttribute::enabled()));

    Ok(())
}

fn skip_translation_attribute_round_trips() {
    let attributed = TranslationAttributedString::new("Leave cargo untouched")
        .with_skip_translation_for_substring("cargo")
        .expect("substring should exist");

    assert_eq!(TranslationAttributes::translation(), TranslationAttributes);
    assert_eq!(
        TranslationAttributes::skips_translation(true),
        SkipTranslationAttribute::enabled()
    );
    assert_eq!(SkipTranslationAttribute::NAME, "Translation.DoNotTranslate");
    assert!(bool::from(SkipTranslationAttribute::enabled()));
    assert_eq!(attributed.skip_translation_runs().len(), 1);

    let json = serde_json::to_string(&attributed).expect("attributed text should serialize");
    let decoded: TranslationAttributedString =
        serde_json::from_str(&json).expect("attributed text should deserialize");
    assert_eq!(decoded, attributed);
}
