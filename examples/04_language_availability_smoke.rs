use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let availability = LanguageAvailability::new()?;
    let languages = availability.supported_language_objects()?;
    let pair = LanguagePair::between("en", "es");

    println!("supported languages: {}", languages.len());
    println!(
        "pair status: {:?}",
        availability.status_for_language_pair(&pair)?
    );
    println!(
        "text status: {:?}",
        availability.status_for_text_in_language("hello world", pair.target())?
    );

    Ok(())
}
