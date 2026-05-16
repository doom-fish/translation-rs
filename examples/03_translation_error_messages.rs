use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;

    match session.cancel() {
        Ok(()) => {}
        Err(error @ TranslationError::UnavailableOnThisMacOS(_)) => {
            println!("cancel unavailable: {error}");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    }

    match session.translate("hello world") {
        Ok(response) => println!("unexpected response: {}", response.target_text()),
        Err(error) => {
            println!("description: {}", error.error_description());
            println!("failure reason: {:?}", error.failure_reason());
        }
    }

    Ok(())
}
