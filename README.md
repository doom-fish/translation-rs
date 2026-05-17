# translation-rs

Safe Rust bindings for Apple's `Translation.framework` on macOS, plus typed language helpers for canonical language identifiers, language pairs, translation configuration state, translation responses, translation errors, and NaturalLanguage-backed language recognition.

> **Status:** v0.3.0 covers all public `Translation.framework` symbols in the current macOS SDK and adds a Tier-1 `async_api` module for Future-based translation and availability workflows.

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

## Async API

Enable the `async` feature to use executor-agnostic futures backed by Swift `Task` thunks and `doom-fish-utils` completion helpers:

```toml
translation-rs = { version = "0.3", features = ["async"] }
```

The async surface currently includes:

- `AsyncTranslationSession::translate`
- `AsyncTranslationSession::translations`
- `AsyncTranslationSession::prepare_translation`
- `AsyncLanguageAvailability::status`
- `AsyncLanguageAvailability::supported_languages`

See `examples/10_async_translate.rs` and `examples/11_async_availability.rs` for end-to-end usage with `pollster::block_on`.

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
- `10_async_translate` *(requires `--features async`)*
- `11_async_availability` *(requires `--features async`)*

Run the sync examples with:

```bash
for ex in 01_language_roundtrip 02_language_pair_roundtrip 03_translation_error_messages 04_language_availability_smoke 05_translation_configuration_lifecycle 06_translation_response_roundtrip 07_language_recognition_smoke 08_translation_session_smoke 09_framework_smoke; do
  cargo run --example "$ex"
done
```

Run the async examples with:

```bash
cargo run --features async --example 10_async_translate
cargo run --features async --example 11_async_availability
```

## Coverage

See [COVERAGE.md](COVERAGE.md) for the API-by-API audit against the active macOS SDK.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
