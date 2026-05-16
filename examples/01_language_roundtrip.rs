use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = Language::new("en");
    let canonical = language.canonicalized()?;
    println!("{language} -> {canonical}");
    Ok(())
}
