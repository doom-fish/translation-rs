use translation::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("detect_language: {:?}", detect_language("hello world")?);
    println!(
        "recognize_language: {:?}",
        recognize_language("hello world")?
    );
    Ok(())
}
