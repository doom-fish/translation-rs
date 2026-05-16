# translation-rs

Safe Rust bindings for Apple's `Translation.framework` on macOS.

> **Status:** v0.1.0 covers `LanguageAvailability`, manual `TranslationSession` translation APIs, batch translation, `prepare_translation`, and NaturalLanguage-backed language detection.

## Quick start

```rust,no_run
use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let availability = LanguageAvailability::new()?;
    println!(
        "en → es status: {:?}",
        availability.status_for_pair("en", "es")?
    );

    let session = TranslationSession::new(
        TranslationSessionConfiguration::new("en", "es"),
    )?;

    match session.translate("hello world") {
        Ok(response) => println!("{}", response.target_text()),
        Err(error) => println!("translation unavailable: {error}"),
    }

    Ok(())
}
```

## Highlights

- `LanguageAvailability::supported_languages`, `status_for_pair`, and `status_for_text`
- `TranslationSessionConfiguration`, `TranslationSession`, `translate`, `translate_batch`, and `prepare_translation`
- `TranslationRequest` / `TranslationResponse`
- `detect_language` fallback powered by `NLLanguageRecognizer`

## Availability

- `LanguageAvailability` is available on macOS 15+.
- In the current public SDK, manual `TranslationSession` construction is available on macOS 26+ via `TranslationSession(installedSource:target:)`.
- This crate guards both surfaces at runtime and returns a structured `TranslationError` when the OS is too old.

## Smoke example

Run the framework smoke example with:

```bash
cargo run --all-features --example 02_framework_smoke
```

It prints supported languages, shows the `en → es` availability state, attempts a translation of `"hello world"`, and degrades cleanly if the required language pack is not installed.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
