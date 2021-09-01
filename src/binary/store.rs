use crate::binary::impls::{Block, V2};
use crate::font::*;

use super::constants::*;
use super::impls::{Magic, C, V1, V3};
use super::pack::{Pack, PackDyn, PackDynLen, PackLen};

use std::io;

pub fn to_writer<W: io::Write>(mut writer: W, font: &Font) -> crate::Result<()> {
    let vec = to_vec(font)?;
    writer.write_all(&vec)?;
    Ok(())
}

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
    dyn_len: DynLen,
    version: u8,
    strict: bool,
}

impl<'a> StoreAssist<'a> {
    fn v3(font: &'a Font, strict: bool) -> Self {
        // Initialize Vec with correct capacity to avoid reallocations/ slack.
        let dyn_len = DynLen::v3(font);
        Self { dst: Vec::with_capacity(dyn_len.sum()), font, dyn_len, version: 3, strict }
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
        self.block(INFO, self.dyn_len.info as u32)?;
        match self.version {
            2 | 3 => PackDyn::<V2>::pack_dyn(&self.font.info, &mut self.dst)?,
            _ => unreachable!(),
        };
        Ok(())
    }

    fn common(&mut self) -> crate::Result<()> {
        self.block(COMMON, self.dyn_len.common as u32)?;
        match self.version {
            3 => Pack::<V3>::pack(&self.font.common, &mut self.dst)?,
            _ => unreachable!(),
        };
        Ok(())
    }

    fn pages(&mut self) -> crate::Result<()> {
        self.block(PAGES, self.dyn_len.pages as u32)?;
        match self.version {
            1 | 2 | 3 => {
                let mut len = 0;
                for (id, file) in self.font.pages.iter().enumerate() {
                    if self.strict {
                        if id == 0 {
                            len = file.len();
                        } else if file.len() != len {
                            return Err(crate::Error::IncongruentPageFileLen { line: None });
                        }
                    }
                    PackDyn::<C>::pack_dyn(&file.as_str(), &mut self.dst)?;
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    fn chars(&mut self) -> crate::Result<()> {
        self.block(CHARS, self.dyn_len.chars as u32)?;
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

    fn kerning_pairs(&mut self) -> crate::Result<()> {
        self.block(KERNING_PAIRS, self.dyn_len.kernings as u32)?;
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

#[derive(Debug, Clone, Copy)]
struct DynLen {
    info: usize,
    common: usize,
    pages: usize,
    chars: usize,
    kernings: usize,
}

impl DynLen {
    fn v3(font: &Font) -> Self {
        Self {
            info: PackDynLen::<V2>::dyn_len(&font.info),
            common: <Common as PackLen<V3>>::PACK_LEN,
            pages: font.pages.iter().map(PackDynLen::<C>::dyn_len).sum(),
            chars: <Char as PackLen<V1>>::PACK_LEN * font.chars.len(),
            kernings: <Kerning as PackLen<V1>>::PACK_LEN * font.kernings.len(),
        }
    }

    fn sum(&self) -> usize {
        self.info + self.common + self.pages + self.chars + self.kernings
    }
}
