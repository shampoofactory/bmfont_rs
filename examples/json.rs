use std::error::Error;
use std::result::Result;

#[cfg(feature = "serde")]
use bmfont_rs::Font;

/// Json Serde import/ export.
///
/// Call with feature `serde`.
///
/// `cargo run --example json --features serde`
///
/// By default bool values are serialized as bool. However, at least one BMFont Json parser uses
/// integers for bool (0 or 1). To mimic this behavior we can pass the `serde_boolint` feature.
///
/// `cargo run --example json --features "serde, serde_boolint"`
#[cfg(feature = "serde")]
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let src = include_bytes!("../data/small.txt");
    let font = bmfont_rs::text::from_bytes(src)?;

    // Export to Json string and print.
    let json = serde_json::ser::to_string_pretty(&font)?;
    println!("{}", json);

    // Convert back to a Font and check for equality.
    let font_2 = serde_json::de::from_str(&json)?;
    assert_eq!(font, font_2);

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
