use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;
    println!("source: {:?}", session.source_language());
    println!("target: {:?}", session.target_language());

    match session.can_request_downloads() {
        Ok(value) => println!("can_request_downloads: {value}"),
        Err(error) => println!("can_request_downloads unavailable: {error}"),
    }

    match session.is_ready() {
        Ok(value) => println!("is_ready: {value}"),
        Err(error) => println!("is_ready unavailable: {error}"),
    }

    let requests = [
        TranslationRequest::new("hello world"),
        TranslationRequest::new("good morning").with_client_identifier("second"),
    ];

    match session.translate_batch_streaming(&requests) {
        Ok(stream) => match stream.collect_all() {
            Ok(responses) => println!("streamed responses: {}", responses.len()),
            Err(error) => println!("streaming iteration unavailable: {error}"),
        },
        Err(error) => println!("streaming batch unavailable: {error}"),
    }

    match session.prepare_translation() {
        Ok(()) => println!("prepare_translation: ready"),
        Err(error) => println!("prepare_translation: {error}"),
    }

    match session.translate("hello world") {
        Ok(response) => println!("single translation: {}", response.target_text()),
        Err(error) => println!("single translation unavailable: {error}"),
    }

    Ok(())
}
