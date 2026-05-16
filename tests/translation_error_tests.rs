use translation::{TranslationError, TranslationSession, TranslationSessionConfiguration};

fn main() -> Result<(), TranslationError> {
    cancelled_sessions_preserve_failure_reason()
}

fn cancelled_sessions_preserve_failure_reason() -> Result<(), TranslationError> {
    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;

    match session.cancel() {
        Ok(()) => {}
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        Err(error) => return Err(error),
    }

    let error = match session.translate("hello world") {
        Err(error @ TranslationError::AlreadyCancelled(_)) => error,
        Err(TranslationError::UnavailableOnThisMacOS(_)) => return Ok(()),
        other => panic!("expected an already-cancelled error, got {other:?}"),
    };

    assert_eq!(error.error_description(), "Unable to Translate");
    assert_eq!(
        error.failure_reason(),
        Some("Translation was already cancelled.")
    );
    Ok(())
}
