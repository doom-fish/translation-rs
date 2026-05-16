use translation::{detect_language, recognize_language, TranslationError};

fn main() -> Result<(), TranslationError> {
    recognizes_english_text()
}

fn recognizes_english_text() -> Result<(), TranslationError> {
    let detected = detect_language("hello world")?.expect("expected an English language tag");
    assert!(detected.starts_with("en"));

    let recognized = recognize_language("hello world")?.expect("expected an English language tag");
    assert!(recognized.identifier().starts_with("en"));
    Ok(())
}
