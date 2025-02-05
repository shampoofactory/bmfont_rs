use crate::builder::FontBuilder;
use crate::font::Font;
use crate::LoadSettings;

use std::io;

/// Load JSON format font.
///
/// Load a font from the specified JSON format [str].
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
/// use std::fs;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let mut src = fs::read_to_string("font.json")?;
///     let font = bmfont_rs::json::from_str(&src)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_str(src: &str) -> crate::Result<Font> {
    from_str_ext(src, &Default::default())
}

/// Load JSON format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_str_ext(src: &str, settings: &LoadSettings) -> crate::Result<Font> {
    let font = serde_json::de::from_str(src).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "json".to_owned(),
        err: e.to_string(),
    })?;
    FontBuilder::with_font(font, settings).build()
}

/// Load JSON format font.
///
/// Load a font from the specified JSON format byte slice.
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
/// use std::fs;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let mut buf = fs::read("font.json")?;
///     let font = bmfont_rs::json::from_bytes(&buf)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    from_bytes_ext(bytes, &Default::default())
}

/// Load JSON format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_bytes_ext(bytes: &[u8], settings: &LoadSettings) -> crate::Result<Font> {
    from_str_ext(
        std::str::from_utf8(bytes).map_err(|e| crate::Error::Parse {
            line: None,
            entity: "font".to_owned(),
            err: e.to_string(),
        })?,
        settings,
    )
}

/// Read JSON format font.
///
/// Read a font from the specified JSON format reader.
/// This method buffers data internally, a buffered reader is not needed.
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
///     let mut f = File::open("font.json")?;
///     let font = bmfont_rs::json::from_reader(f)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_reader<R: io::Read>(reader: R) -> crate::Result<Font> {
    from_reader_ext(reader, &Default::default())
}

/// Read JSON format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_reader_ext<R: io::Read>(mut reader: R, settings: &LoadSettings) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes_ext(&vec, settings)
}
