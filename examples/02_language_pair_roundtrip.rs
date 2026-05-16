use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pair = LanguagePair::between("en", "es").canonicalized()?;
    println!(
        "{} -> {:?}",
        pair.source(),
        pair.target().map(Language::identifier)
    );
    Ok(())
}
