use translation::{Language, TranslationError};

fn main() -> Result<(), TranslationError> {
    canonicalizes_language_identifiers()
}

fn canonicalizes_language_identifiers() -> Result<(), TranslationError> {
    let language = Language::new("en");
    let canonical = language.canonicalized()?;
    assert!(canonical.identifier().starts_with("en"));
    assert!(canonical.to_string().starts_with("en"));
    Ok(())
}
