use std::error::Error;
use std::result::Result;

#[cfg(feature = "xml")]
use bmfont_rs::Font;

/// XML Serde import/ export.
///
/// Call with feature `xml`.
///
/// `cargo run --example xml --features xml`
#[cfg(feature = "xml")]
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let xml = include_str!("../data/small.xml");
    println!("XML in:");
    println!("{}\n", xml);

    // Import.
    let font = bmfont_rs::xml::from_str(xml)?;
    println!("Font:");
    println!("{:#?}\n", font);

    // Export.
    let xml = bmfont_rs::xml::to_string(&font)?;
    println!("XML out:");
    println!("{}\n", xml);

    Ok(())
}

/// Not the real main. We expect to have the `xml` feature.
#[cfg(not(feature = "xml"))]
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Error, use:");
    eprintln!("cargo run --example xml --features xml");
    Ok(())
}
