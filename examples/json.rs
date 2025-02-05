use std::error::Error;
use std::result::Result;

#[cfg(feature = "json")]

/// JSON Serde import/ export.
///
/// Call with feature `json`.
///
/// `cargo run --example json --features json`
#[cfg(feature = "json")]
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let json = include_str!("../data/ok/small.json");
    println!("JSON in:");
    println!("{}\n", json);

    // Import.
    let font = bmfont_rs::json::from_str(json)?;
    println!("Font:");
    println!("{:#?}\n", font);

    // Export.
    let json = bmfont_rs::json::to_string_pretty(&font)?;
    println!("JSON pretty out:");
    println!("{}\n", json);

    Ok(())
}

/// Not the real main. We expect to have the `json` feature.
#[cfg(not(feature = "json"))]
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Error, use:");
    eprintln!("cargo run --example json --features json");
    Ok(())
}
