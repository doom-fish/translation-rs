use translation::prelude::*;

fn main() {
    let mut configuration = TranslationConfiguration::new()
        .with_source("en")
        .with_target("es");

    println!("source: {:?}", configuration.source_identifier());
    println!("target: {:?}", configuration.target_identifier());
    println!("version: {}", configuration.version());

    configuration.invalidate();
    configuration.clear_target();

    println!("invalidated version: {}", configuration.version());
    println!("language pair: {:?}", configuration.language_pair());
}
