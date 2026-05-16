use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Translation.framework smoke ==");

    let source = Language::new("en").canonicalized()?;
    let target = Language::new("es").canonicalized()?;
    let pair = LanguagePair::between(source.clone(), target.clone()).canonicalized()?;

    let availability = LanguageAvailability::new()?;
    let supported_languages = availability.supported_language_objects()?;
    println!("supported languages: {}", supported_languages.len());
    println!(
        "sample languages: {:?}",
        supported_languages.iter().take(10).collect::<Vec<_>>()
    );
    println!(
        "pair status: {:?}",
        availability.status_for_language_pair(&pair)?
    );
    println!(
        "text status: {:?}",
        availability.status_for_text_in_language("hello world", pair.target())?
    );

    let mut configuration = TranslationConfiguration::new()
        .with_source(source)
        .with_target(target);
    println!("configuration version: {}", configuration.version());
    configuration.invalidate();
    println!("configuration invalidated: {}", configuration.version());

    let session = TranslationSession::from_translation_configuration(&configuration)?;

    match session.can_request_downloads() {
        Ok(value) => println!("can_request_downloads: {value}"),
        Err(error) => println!("can_request_downloads unavailable: {error}"),
    }

    match session.is_ready() {
        Ok(value) => println!("is_ready: {value}"),
        Err(error) => println!("is_ready unavailable: {error}"),
    }

    match session.prepare_translation() {
        Ok(()) => println!("prepare_translation: ready"),
        Err(error) => println!("prepare_translation: {error}"),
    }

    match session.translate("hello world") {
        Ok(response) => println!(
            "single translation: {} -> {}",
            response.source_text(),
            response.target_text()
        ),
        Err(error) => println!("single translation unavailable: {error}"),
    }

    let batch_requests = [
        TranslationRequest::new("hello world"),
        TranslationRequest::new("doom fish").with_client_identifier("doom-fish"),
    ];
    match session.translate_batch(&batch_requests) {
        Ok(responses) => {
            println!("batch responses: {}", responses.len());
            for response in responses {
                println!(
                    "batch translation [{}]: {} -> {}",
                    response.client_identifier().unwrap_or("<none>"),
                    response.source_text(),
                    response.target_text(),
                );
            }
        }
        Err(error) => println!("batch translation unavailable: {error}"),
    }

    match session.translate_batch_streaming(&batch_requests) {
        Ok(stream) => match stream.collect_all() {
            Ok(responses) => println!("streamed batch responses: {}", responses.len()),
            Err(error) => println!("streamed batch unavailable: {error}"),
        },
        Err(error) => println!("streamed batch unavailable: {error}"),
    }

    println!(
        "recognized language: {:?}",
        recognize_language("hello world")?
    );
    println!("detected language: {:?}", detect_language("hello world")?);

    Ok(())
}
