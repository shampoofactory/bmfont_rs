use crate::builder::tags::{Tag, Tags};
use crate::builder::FontBuilder;
use crate::font::Font;
use crate::tagged_attributes::TaggedAttributes;

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
    from_bytes(src.as_bytes())
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
    FontBuilderFnt::default().load_bytes(bytes)?.build()
}

/// Load text format font with relaxed constraints check.
///
/// This function is similar to [from_bytes], but it allows somewhat malformed files
/// to still be loaded. For example when the specified character count differs from
/// the actual amount of characters in file it will load all of them and not return
/// an error.
pub fn from_bytes_relaxed(bytes: &[u8]) -> crate::Result<Font> {
    FontBuilderFnt::relaxed().load_bytes(bytes)?.build()
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
pub fn from_reader<R: io::Read>(mut reader: R) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes(&vec)
}

pub struct FontBuilderFnt {
    builder: FontBuilder,
}

impl FontBuilderFnt {
    pub fn relaxed() -> Self {
        Self { builder: FontBuilder::relaxed() }
    }

    pub fn load_bytes(mut self, bytes: &[u8]) -> crate::Result<FontBuilder> {
        let mut attributes = TaggedAttributes::from_bytes(bytes);
        while let Some(Tag { tag, line }) = attributes.next_tag()? {
            match tag {
                b"info" => self.builder.set_info(line, &mut attributes),
                b"common" => self.builder.set_common(line, &mut attributes),
                b"page" => self.builder.page(line, &mut attributes),
                b"chars" => self.builder.chars(line, &mut attributes),
                b"char" => self.builder.char(&mut attributes),
                b"kernings" => self.builder.kernings(line, &mut attributes),
                b"kerning" => self.builder.kerning(&mut attributes),
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

impl Default for FontBuilderFnt {
    fn default() -> Self {
        Self { builder: Default::default() }
    }
}
