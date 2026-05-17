use translation::{Language, LanguagePair, TranslationError};

fn main() -> Result<(), TranslationError> {
    canonicalizes_language_pairs()
}

fn canonicalizes_language_pairs() -> Result<(), TranslationError> {
    let pair = LanguagePair::between("en", "es").canonicalized()?;
    assert!(pair.source().identifier().starts_with("en"));
    assert!(pair
        .target()
        .map(Language::identifier)
        .is_some_and(|tag| tag.starts_with("es")));

    let without_target = LanguagePair::new("en", None).canonicalized()?;
    assert!(without_target.source().identifier().starts_with("en"));
    assert_eq!(without_target.target(), None);
    Ok(())
}
