use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = TranslationResponse::new("en", "es", "hello world", "hola mundo")
        .with_client_identifier("greeting");
    let json = serde_json::to_string(&response)?;
    let decoded: TranslationResponse = serde_json::from_str(&json)?;

    println!("json: {json}");
    println!(
        "decoded: {} -> {} ({:?})",
        decoded.source_text(),
        decoded.target_text(),
        decoded.client_identifier()
    );

    Ok(())
}
