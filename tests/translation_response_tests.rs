use translation::TranslationResponse;

fn main() {
    response_constructor_round_trips_through_json();
}

fn response_constructor_round_trips_through_json() {
    let response = TranslationResponse::new("en", "es", "hello world", "hola mundo")
        .with_client_identifier("greeting");

    assert_eq!(response.source_language(), "en");
    assert_eq!(response.target_language(), "es");
    assert_eq!(response.client_identifier(), Some("greeting"));

    let json = serde_json::to_string(&response).expect("response should serialize");
    let decoded: TranslationResponse =
        serde_json::from_str(&json).expect("response should deserialize");
    assert_eq!(decoded, response);
}
