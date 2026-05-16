use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Translation.framework smoke ==");

    let availability = LanguageAvailability::new()?;
    let supported_languages = availability.supported_languages()?;
    println!("supported languages: {}", supported_languages.len());
    println!(
        "sample languages: {:?}",
        supported_languages.iter().take(10).collect::<Vec<_>>()
    );

    let en_es_status = availability.status_for_pair("en", "es")?;
    println!("en -> es status: {en_es_status:?}");

    let detected_language = detect_language("hello world")?;
    println!("detected language for 'hello world': {detected_language:?}");

    let text_status = availability.status_for_text("hello world", "es")?;
    println!("status for text -> es: {text_status:?}");

    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;

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

    Ok(())
}
