use std::error::Error;
use std::result::Result;

/// Binary to text conversion.
///
/// `cargo run --example binary_to_text`
fn main() -> Result<(), Box<dyn Error>> {
    // Load binary font data.
    let bin = include_bytes!("../data/ok/small.bin");

    println!("Binary in:");
    bin.as_ref().chunks(0x10).for_each(|u| println!("{:02X?}", u));

    // Import binary font.
    let font = bmfont_rs::binary::from_bytes(bin)?;
    println!("Font:");
    println!("{:#?}\n", font);

    // Export text font data.
    let text = bmfont_rs::text::to_string(&font)?;
    println!("Text out:");
    println!("{}", text);

    Ok(())
}
