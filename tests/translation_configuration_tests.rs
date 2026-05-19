use translation::{TranslationConfiguration, TranslationStrategy};

fn main() {
    tracks_source_target_and_version();
}

fn tracks_source_target_and_version() {
    let mut configuration = TranslationConfiguration::new()
        .with_source("en")
        .with_target("es");

    assert_eq!(configuration.source_identifier(), Some("en"));
    assert_eq!(configuration.target_identifier(), Some("es"));
    assert_eq!(configuration.preferred_strategy(), TranslationStrategy::HighFidelity);
    configuration.set_preferred_strategy(TranslationStrategy::LowLatency);
    assert_eq!(configuration.preferred_strategy(), TranslationStrategy::LowLatency);
    assert_eq!(configuration.version(), 0);

    configuration.invalidate();
    assert_eq!(configuration.version(), 1);

    configuration.clear_target();
    assert_eq!(configuration.target_identifier(), None);

    let pair = configuration
        .language_pair()
        .expect("source should remain present");
    assert_eq!(pair.source().identifier(), "en");
    assert_eq!(pair.target(), None);
}
