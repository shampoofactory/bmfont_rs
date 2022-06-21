use std::error::Error;
use std::result::Result;

/// Text import/ export.
///
/// `cargo run --example text`
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let src = include_str!("../data/ok/small.txt");

    println!("Text in:");
    println!("{}", &src);

    // Import
    let font = bmfont_rs::text::from_str(src)?;
    println!("Font:");
    println!("{:#?}\n", font);

    // Export to text string and print.
    let txt = bmfont_rs::text::to_string(&font)?;

    // Export.
    println!("Text out:");
    println!("{}", txt);

    Ok(())
}
