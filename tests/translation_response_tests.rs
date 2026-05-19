use translation::{TranslationAttributedString, TranslationResponse};

fn main() {
    response_constructor_round_trips_through_json();
}

fn response_constructor_round_trips_through_json() {
    let attributed_source = TranslationAttributedString::new("hello world")
        .with_skip_translation_for_substring("world")
        .expect("substring should exist");
    let response = TranslationResponse::new("en", "es", "hello world", "hola mundo")
        .with_attributed_source_text(attributed_source.clone())
        .with_attributed_target_text("hola mundo")
        .with_client_identifier("greeting");

    assert_eq!(response.source_language(), "en");
    assert_eq!(response.target_language(), "es");
    assert_eq!(response.client_identifier(), Some("greeting"));
    assert_eq!(response.attributed_source_text(), Some(&attributed_source));
    assert_eq!(
        response
            .attributed_target_text()
            .map(TranslationAttributedString::text),
        Some("hola mundo")
    );

    let json = serde_json::to_string(&response).expect("response should serialize");
    let decoded: TranslationResponse =
        serde_json::from_str(&json).expect("response should deserialize");
    assert_eq!(decoded, response);
}
