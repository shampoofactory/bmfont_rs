use crate::binary::impls::{Block, V2};
use crate::font::*;

use super::constants::*;
use super::impls::{Magic, C, V1, V3};
use super::pack::{Pack, PackDyn, PackDynLen, PackLen};

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
    let mut assist = StoreAssist::v3(font, true);
    assist.init()?;
    assist.info()?;
    assist.common()?;
    assist.pages()?;
    assist.chars()?;
    assist.kerning_pairs()?;
    Ok(assist.dst)
}

struct StoreAssist<'a> {
    dst: Vec<u8>,
    font: &'a Font,
    version: u8,
    strict: bool,
}

impl<'a> StoreAssist<'a> {
    fn v3(font: &'a Font, strict: bool) -> Self {
        // Initialize Vec with correct capacity to avoid reallocations/ slack.
        Self {
            dst: Vec::with_capacity(<Font as PackDynLen<V3>>::dyn_len(font)),
            font,
            version: 3,
            strict,
        }
    }

    fn init(&mut self) -> crate::Result<()> {
        let magic = Magic::new(self.version);
        <Magic as Pack>::pack(&magic, &mut self.dst)?;
        Ok(())
    }

    fn info(&mut self) -> crate::Result<()> {
        if self.strict {
            self.font.info.check_encoding()?;
        }
        self.block(INFO, <Info as PackDynLen<V2>>::dyn_len(&self.font.info) as u32)?;
        match self.version {
            2 | 3 => PackDyn::<V2>::pack_dyn(&self.font.info, &mut self.dst)?,
            _ => unreachable!(),
        };
        Ok(())
    }

    fn common(&mut self) -> crate::Result<()> {
        self.block(COMMON, <Common as PackLen<V3>>::PACK_LEN as u32)?;
        match self.version {
            3 => Pack::<V3>::pack(&self.font.common, &mut self.dst)?,
            _ => unreachable!(),
        };
        Ok(())
    }

    #[allow(clippy::manual_range_patterns)]
    fn pages(&mut self) -> crate::Result<()> {
        self.block(PAGES, <Vec<String> as PackDynLen<C>>::dyn_len(&self.font.pages) as u32)?;
        match self.version {
            1 | 2 | 3 => {
                let mut len = 0;
                for (id, file) in self.font.pages.iter().enumerate() {
                    if self.strict {
                        if id == 0 {
                            len = file.len();
                        } else if file.len() != len {
                            return Err(crate::Error::IncongruentPageNameLen { line: None });
                        }
                    }
                    PackDyn::<C>::pack_dyn(&file.as_str(), &mut self.dst)?;
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[allow(clippy::manual_range_patterns)]
    fn chars(&mut self) -> crate::Result<()> {
        self.block(CHARS, <Vec<Char> as PackDynLen<V1>>::dyn_len(&self.font.chars) as u32)?;
        match self.version {
            1 | 2 | 3 => {
                for char in self.font.chars.iter() {
                    Pack::<V1>::pack(char, &mut self.dst)?;
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[allow(clippy::manual_range_patterns)]
    fn kerning_pairs(&mut self) -> crate::Result<()> {
        if self.font.kernings.is_empty() {
            return Ok(());
        }
        self.block(
            KERNING_PAIRS,
            <Vec<Kerning> as PackDynLen<V1>>::dyn_len(&self.font.kernings) as u32,
        )?;
        match self.version {
            1 | 2 | 3 => {
                for kerning in self.font.kernings.iter() {
                    Pack::<V1>::pack(kerning, &mut self.dst)?;
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    fn block(&mut self, id: u8, len: u32) -> crate::Result<()> {
        let block = Block::new(id, len);
        <Block as Pack>::pack(&block, &mut self.dst)?;
        Ok(())
    }
}
