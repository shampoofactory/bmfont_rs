use crate::font::*;

use super::impls::V3;
use super::pack::{PackDyn, PackDynLen};

use std::io;

/// Write binary format font.
///
/// Write a font to the specified writer in binary format.
/// This method buffers data internally, a buffered writer is not needed.
///
/// N.B. The binary format is strict.
/// Additional errors may be thrown in comparison to other formats.
///
/// # Errors
///
/// * [Error](crate::Error) detailing the nature of any errors.
///
/// # Example
///
/// ```no_run
/// use std::io;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let font = bmfont_rs::Font::default();
///     let mut writer = File::create("font.bin")?;
///     bmfont_rs::binary::to_writer(&mut writer, &font)?;
///     Ok(())
/// }
/// ```
pub fn to_writer<W: io::Write>(mut writer: W, font: &Font) -> crate::Result<()> {
    let vec = to_vec(font)?;
    writer.write_all(&vec)?;
    Ok(())
}

/// Store binary format font.
///
/// Store a font into a [Vec] in binary format.
///
/// N.B. The binary format is strict.
/// Additional errors may be thrown in comparison to other formats.
///
/// # Errors
///
/// * [Error](crate::Error) detailing the nature of any errors.
///
/// # Example
///
/// ```no_run
/// fn main() -> bmfont_rs::Result<()> {
///     let font = bmfont_rs::Font::default();
///     let vec = bmfont_rs::binary::to_vec(&font)?;
///     println!("{:02X?}", font);
///     Ok(())
/// }
/// ```
pub fn to_vec(font: &Font) -> crate::Result<Vec<u8>> {
    check_page_names(&font.pages)?;
    check_value(&font.info.face)?;
    let dyn_len = PackDynLen::<V3>::dyn_len(font);
    let mut dst = Vec::with_capacity(dyn_len);
    PackDyn::<V3>::pack_dyn(font, &mut dst)?;
    Ok(dst)
}

fn check_page_names(pages: &[String]) -> crate::Result<()> {
    let mut len = None;
    for page in pages {
        let page_len = page.len();
        if *len.get_or_insert(page_len) != page_len {
            return Err(crate::Error::IncongruentPageNameLen { line: None });
        }
        check_value(page)?;
    }
    Ok(())
}

fn check_value(value: &str) -> crate::Result<&str> {
    for c in value.chars() {
        if c == '\x00' {
            return Err(crate::Error::UnsupportedValueEncoding {
                path: "binary".to_owned(),
                value: value.to_owned(),
            });
        }
    }
    Ok(value)
}
