use std::error::Error;
use std::result::Result;

/// Binary import/ export.
///
/// `cargo run --example binary`
fn main() -> Result<(), Box<dyn Error>> {
    // Load some sample font data.
    let bin = include_bytes!("../data/ok/small.bin");

    println!("Binary in:");
    bin.as_ref().chunks(0x10).for_each(|u| println!("{:02X?}", u));

    // Import.
    let font = bmfont_rs::binary::from_bytes(bin)?;
    println!("Font:");
    println!("{:#?}\n", font);

    // Export to text string and print.
    let bin = bmfont_rs::binary::to_vec(&font)?;

    // Export.
    println!("Binary out:");
    bin.as_slice().chunks(0x10).for_each(|u| println!("{:02X?}", u));

    Ok(())
}
