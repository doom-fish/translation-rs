# translation-rs

Safe Rust bindings for Apple's `Translation.framework` on macOS, plus typed language helpers for canonical language identifiers, language pairs, translation configuration state, translation responses, translation errors, and NaturalLanguage-backed language recognition.

> **Status:** v0.2.0 covers all public `Translation.framework` symbols in the current macOS SDK and organizes the crate into eight logical areas: `TranslationSession`, `LanguageAvailability`, `TranslationConfiguration`, `TranslationResponse`, `TranslationError`, `Language`, `LanguagePair`, and `LanguageRecognition`.

## Quick start

```rust,no_run
use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Language::new("en").canonicalized()?;
    let target = Language::new("es").canonicalized()?;
    let pair = LanguagePair::between(source.clone(), target.clone()).canonicalized()?;

    let availability = LanguageAvailability::new()?;
    println!(
        "{pair:?} status: {:?}",
        availability.status_for_language_pair(&pair)?
    );

    let mut configuration = TranslationConfiguration::new()
        .with_source(source)
        .with_target(target);
    configuration.invalidate();

    let session = TranslationSession::from_translation_configuration(&configuration)?;
    match session.translate("hello world") {
        Ok(response) => println!("{}", response.target_text()),
        Err(error) => println!("translation unavailable: {error}"),
    }

    println!("recognized: {:?}", recognize_language("hello world")?);
    Ok(())
}
```

## Highlights

- `Language` and `LanguagePair` wrappers that canonicalize `Locale.Language` identifiers through the Swift bridge.
- `LanguageAvailability` access to `supportedLanguages`, pair status, and text status, including optional target-language queries.
- `TranslationConfiguration` for mutable source/target configuration state and invalidation version tracking.
- `TranslationSession` with `can_request_downloads`, `is_ready`, `cancel`, eager batch translation, and streaming `translate(batch:)` iteration.
- `TranslationRequest` setters plus `TranslationResponse` constructors, serde support, and typed language accessors.
- `TranslationError` variants that preserve both `errorDescription` and `failureReason` from `Translation.framework` errors.
- `recognize_language` / `detect_language` fallback powered by `NLLanguageRecognizer`.

## Availability

- The crate weak-links `Translation.framework`, so the typed `Language`, `LanguagePair`, `TranslationConfiguration`, `TranslationResponse`, and `LanguageRecognition` helpers work on macOS 14+.
- `LanguageAvailability` requires macOS 15+.
- Manual `TranslationSession` construction is currently available on macOS 26+ through `TranslationSession(installedSource:target:)`; the crate surfaces those APIs and returns structured `TranslationError` values on older systems.

## Examples

The crate ships numbered examples for every logical area plus an end-to-end framework smoke test:

- `01_language_roundtrip`
- `02_language_pair_roundtrip`
- `03_translation_error_messages`
- `04_language_availability_smoke`
- `05_translation_configuration_lifecycle`
- `06_translation_response_roundtrip`
- `07_language_recognition_smoke`
- `08_translation_session_smoke`
- `09_framework_smoke`

Run them all with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Coverage

See [COVERAGE.md](COVERAGE.md) for the API-by-API audit against the active macOS SDK.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
