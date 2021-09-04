use crate::builder::tags::{Tag, Tags};
use crate::builder::FontBuilder;
use crate::font::Font;
use crate::tagged_attributes::TaggedAttributes;

use std::io;

pub fn from_str(src: &str) -> crate::Result<Font> {
    from_bytes(src.as_bytes())
}

pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    FontBuilderFnt::default().load_bytes(bytes)?.build()
}

pub fn from_reader<R: io::Read>(mut reader: R) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes(&vec)
}

pub struct FontBuilderFnt {
    builder: FontBuilder,
}

impl FontBuilderFnt {
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

#[inline(always)]
pub fn utf8_string(line: Option<usize>, bytes: &[u8]) -> crate::Result<String> {
    String::from_utf8(bytes.into()).map_err(|e| crate::Error::Parse {
        line,
        entity: "tag".to_owned(),
        err: format!("UTF8: {}", e),
    })
}
