use std::error::Error;
use std::result::Result;

#[cfg(feature = "serde")]
use bmfont_rs::Font;

/// JSON Serde import/ export.
///
/// JSON is a non-standard BMFont import/ export format. Other implementations may vary in output.
///
/// Call with feature `serde`.
///
/// `cargo run --example json --features serde`
///
/// By default bool values are serialized as bool. However, at least one BMFont JSON parser uses
/// integers for bool (0 or 1). To mimic this behavior we can pass the `serde_boolint` feature.
///
/// `cargo run --example json --features "serde, serde_boolint"`
#[cfg(feature = "serde")]
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let text = include_str!("../data/ok/small.txt");
    let font = bmfont_rs::text::from_str(text)?;

    // Export.
    let json = serde_json::ser::to_string_pretty(&font)?;
    println!("JSON out:");
    println!("{}\n", json);

    // Import.
    let font: Font = serde_json::de::from_str(&json)?;
    println!("Font:");
    println!("{:#?}\n", font);

    Ok(())
}

/// Not the real main. We expect to have the `serde` feature.
#[cfg(not(feature = "serde"))]
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Error, use one of:");
    eprintln!("cargo run --example json --features serde");
    eprintln!("cargo run --example json --features \"serde, serde_boolint\"");
    Ok(())
}
