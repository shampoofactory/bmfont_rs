use crate::builder::tags::{Tag, Tags};
use crate::builder::FontBuilder;
use crate::font::Font;
use crate::tagged_attributes::TaggedAttributes;
use crate::LoadSettings;

use std::io;

/// Load text format font.
///
/// Load a font from the specified text format [str].
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
///     let mut src = fs::read_to_string("font.txt")?;
///     let font = bmfont_rs::text::from_str(&src)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_str(src: &str) -> crate::Result<Font> {
    from_str_ext(src, &Default::default())
}

/// Load text format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_str_ext(src: &str, settings: &LoadSettings) -> crate::Result<Font> {
    from_bytes_ext(src.as_bytes(), settings)
}

/// Load text format font.
///
/// Load a font from the specified text format byte slice.
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
///     let mut buf = fs::read("font.txt")?;
///     let font = bmfont_rs::text::from_bytes(&buf)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    from_bytes_ext(bytes, &Default::default())
}

/// Load text format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_bytes_ext(bytes: &[u8], settings: &LoadSettings) -> crate::Result<Font> {
    FontBuilderText::default().load_bytes(bytes)?.build(settings)
}

/// Read text format font.
///
/// Read a font from the specified text format reader.
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
///     let mut f = File::open("font.txt")?;
///     let font = bmfont_rs::text::from_reader(f)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_reader<R: io::Read>(reader: R) -> crate::Result<Font> {
    from_reader_ext(reader, &Default::default())
}

/// Read text format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_reader_ext<R: io::Read>(mut reader: R, settings: &LoadSettings) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes_ext(&vec, settings)
}

#[derive(Debug, Default)]
pub struct FontBuilderText {
    builder: FontBuilder,
}

impl FontBuilderText {
    pub fn load_bytes(mut self, bytes: &[u8]) -> crate::Result<FontBuilder> {
        let mut attributes = TaggedAttributes::from_bytes(bytes);
        while let Some(Tag { tag, line }) = attributes.next_tag()? {
            match tag {
                b"info" => self.builder.set_info_attributes(line, &mut attributes),
                b"common" => self.builder.set_common_attributes(line, &mut attributes),
                b"page" => self.builder.add_page_attributes(&mut attributes),
                b"chars" => self.builder.set_char_count_attributes(line, &mut attributes),
                b"char" => self.builder.add_char_attributes(&mut attributes),
                b"kernings" => self.builder.set_kerning_count_attributes(line, &mut attributes),
                b"kerning" => self.builder.add_kerning_attributes(&mut attributes),
                tag => {
                    let line = Some(attributes.line());
                    let tag = String::from_utf8(tag.into()).map_err(|e| crate::Error::Parse {
                        line,
                        entity: "tag".to_owned(),
                        err: e.to_string(),
                    })?;
                    Err(crate::Error::InvalidTag { line, tag })
                }
            }?;
        }
        Ok(self.builder)
    }
}
