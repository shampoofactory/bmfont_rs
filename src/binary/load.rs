use crate::binary::impls::{Magic, C, V3};
use crate::{font::*, LoadSettings};

use super::constants::*;
use super::impls::{Block, V1, V2};
use super::pack::{self, Unpack, UnpackDyn};

use std::io;

/// Read binary format font.
///
/// Read a font from the specified binary format reader.
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
///     let mut f = File::open("font.bin")?;
///     let font = bmfont_rs::binary::from_reader(f)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_reader<R: io::Read>(reader: R) -> crate::Result<Font> {
    from_reader_ext(reader, &Default::default())
}

/// Read binary format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_reader_ext<R: io::Read>(mut reader: R, settings: &LoadSettings) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes_ext(vec.as_slice(), settings)
}

/// Load binary format font.
///
/// Load a font from the specified binary format byte slice.
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
///     let mut buf = fs::read("font.bin")?;
///     let font = bmfont_rs::binary::from_bytes(&buf)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    from_bytes_ext(bytes, &Default::default())
}

/// Load binary format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_bytes_ext(mut bytes: &[u8], settings: &LoadSettings) -> crate::Result<Font> {
    let magic: Magic = Unpack::<()>::unpack_take(&mut bytes)?;
    let mut builder = FontBuilderBinary::new(bytes, settings, magic.version()?)?;
    builder.load()?;
    builder.build()
}

#[derive(Debug)]
struct FontBuilderBinary<'a> {
    src: &'a [u8],
    _settings: &'a LoadSettings,
    version: u8,
    info: Option<Info>,
    common: Option<Common>,
    pages: Vec<String>,
    chars: Vec<Char>,
    kernings: Vec<Kerning>,
}

impl<'a> FontBuilderBinary<'a> {
    fn new(src: &'a [u8], settings: &'a LoadSettings, version: u8) -> crate::Result<Self> {
        if version == 3 {
            Ok(Self {
                src,
                _settings: settings,
                version,
                info: None,
                common: None,
                pages: Vec::default(),
                chars: Vec::default(),
                kernings: Vec::default(),
            })
        } else {
            Err(crate::Error::UnsupportedBinaryVersion { version })
        }
    }

    fn build(mut self) -> crate::Result<Font> {
        let info = self.info.take().ok_or(crate::Error::NoInfoBlock)?;
        let common = self.common.take().ok_or(crate::Error::NoCommonBlock)?;
        let pages = self.pages;
        if pages.len() != common.pages.into() {
            return Err(crate::Error::PageCountMismatch {
                pages_len: pages.len(),
                common_pages: common.pages,
            });
        }

        Ok(Font::new(info, common, pages, self.chars, self.kernings))
    }

    fn load(&mut self) -> crate::Result<()> {
        while !self.src.is_empty() {
            self.next()?;
        }
        Ok(())
    }

    fn next(&mut self) -> crate::Result<()> {
        let (id, src) = self.block()?;
        match id {
            INFO => self.info(src),
            COMMON => self.common(src),
            PAGES => self.pages(src),
            CHARS => self.chars(src),
            KERNING_PAIRS => self.kerning_pairs(src),
            id => Err(crate::Error::InvalidBinaryBlock { id }),
        }
    }

    fn info(&mut self, src: &[u8]) -> crate::Result<()> {
        if self.info.is_some() {
            return Err(crate::Error::DuplicateInfoBlock { line: None });
        }
        self.info = Some(match self.version {
            2 | 3 => <Info as UnpackDyn<V2>>::unpack_dyn_tight(src)?,
            _ => unreachable!(),
        });
        Ok(())
    }

    fn common(&mut self, src: &[u8]) -> crate::Result<()> {
        if self.common.is_some() {
            return Err(crate::Error::DuplicateCommonBlock { line: None });
        }
        self.common = Some(match self.version {
            3 => <Common as Unpack<V3>>::unpack_tight(src)?,
            _ => unreachable!(),
        });
        Ok(())
    }

    fn pages(&mut self, src: &[u8]) -> crate::Result<()> {
        match self.version {
            1 | 2 | 3 => {
                <String as UnpackDyn<C>>::unpack_dyn_take_all(src, |file| {
                    self.pages.push(file);
                    Ok(())
                })?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn chars(&mut self, src: &[u8]) -> crate::Result<()> {
        match self.version {
            1 | 2 | 3 => {
                <Char as Unpack<V1>>::unpack_take_all(src, |char| {
                    self.chars.push(char);
                    Ok(())
                })?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn kerning_pairs(&mut self, src: &[u8]) -> crate::Result<()> {
        match self.version {
            1 | 2 | 3 => {
                <Kerning as Unpack<V1>>::unpack_take_all(src, |kerning| {
                    self.kernings.push(kerning);
                    Ok(())
                })?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    #[inline(always)]
    fn block(&mut self) -> crate::Result<(u8, &'a [u8])> {
        let Block { id, len } = match self.version {
            1 | 2 | 3 => <Block as Unpack>::unpack_take(&mut self.src)?,
            _ => unreachable!(),
        };
        Ok((id, self.bytes(len as usize)?))
    }

    #[inline(always)]
    fn bytes(&mut self, len: usize) -> crate::Result<&'a [u8]> {
        if len <= self.src.len() {
            let bytes = &self.src[..len];
            self.src = &self.src[len..];
            Ok(bytes)
        } else {
            pack::underflow()
        }
    }
}
