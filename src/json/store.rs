use crate::font::Font;

use std::io;

/// Store JSON format font.
///
/// Store a font into a [String] in JSON format.
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
///     let string = bmfont_rs::json::to_string(&font)?;
///     println!("{}", string);
///     Ok(())
/// }
/// ```
pub fn to_string(font: &Font) -> crate::Result<String> {
    let vec = to_vec(font)?;
    String::from_utf8(vec).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "font".to_owned(),
        err: format!("UTF8: {}", e),
    })
}

/// Store pretty JSON format font.
///
/// Store a font into a [String] in pretty JSON format.
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
///     let string = bmfont_rs::json::to_string_pretty(&font)?;
///     println!("{}", string);
///     Ok(())
/// }
/// ```
pub fn to_string_pretty(font: &Font) -> crate::Result<String> {
    let vec = to_vec_pretty(font)?;
    String::from_utf8(vec).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "font".to_owned(),
        err: format!("UTF8: {}", e),
    })
}

/// Store JSON format font.
///
/// Store a font into a [Vec] in JSON format.
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
///     let vec = bmfont_rs::json::to_vec(&font)?;
///     println!("{:02X?}", font);
///     Ok(())
/// }
/// ```
pub fn to_vec(font: &Font) -> crate::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::default();
    to_writer(&mut vec, font)?;
    Ok(vec)
}

/// Store pretty JSON format font.
///
/// Store a font into a [Vec] in pretty JSON format.
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
///     let vec = bmfont_rs::json::to_vec_pretty(&font)?;
///     println!("{:02X?}", font);
///     Ok(())
/// }
/// ```
pub fn to_vec_pretty(font: &Font) -> crate::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::default();
    to_writer_pretty(&mut vec, font)?;
    Ok(vec)
}

/// Write JSON format font.
///
/// Write a font to the specified writer in JSON format.
/// This method buffers data internally, a buffered writer is not needed.
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
///     let mut writer = File::create("font.json")?;
///     bmfont_rs::json::to_writer(&mut writer, &font)?;
///     Ok(())
/// }
/// ```
pub fn to_writer<W: io::Write>(mut writer: W, font: &Font) -> crate::Result<()> {
    let json =
        serde_json::ser::to_string(&font).map_err(|e| crate::Error::UnsupportedEncoding {
            line: None,
            entity: "json".to_owned(),
            err: e.to_string(),
        })?;
    write!(writer, "{}", json).map_err(Into::into)
}

/// Write JSON pretty format font.
///
/// Write a font to the specified writer in pretty JSON format.
/// This method buffers data internally, a buffered writer is not needed.
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
///     let mut writer = File::create("font.json")?;
///     bmfont_rs::json::to_writer_pretty(&mut writer, &font)?;
///     Ok(())
/// }
/// ```
pub fn to_writer_pretty<W: io::Write>(mut writer: W, font: &Font) -> crate::Result<()> {
    let json = serde_json::ser::to_string_pretty(&font).map_err(|e| {
        crate::Error::UnsupportedEncoding {
            line: None,
            entity: "json".to_owned(),
            err: e.to_string(),
        }
    })?;
    write!(writer, "{}", json).map_err(Into::into)
}
