pub mod attributes;
pub mod tags;

use crate::charset::Charset;
use crate::font::{Char, Chnl, Common, Font, Info, Padding, Page, Spacing};
use crate::font::{Kerning, Packing};
use crate::parse::Parse;
use crate::Error;

use attributes::{Attribute, Attributes};

#[derive(Debug)]
pub struct FontBuilder {
    info: Option<Info>,
    common: Option<Common>,
    pages: Vec<String>,
    chars: Vec<Char>,
    char_count: Option<u32>,
    kernings: Vec<Kerning>,
    kerning_count: Option<u32>,
}

impl FontBuilder {
    pub fn build(mut self) -> crate::Result<Font> {
        if let Some(specified) = self.char_count {
            let realized = self.chars.len();
            if specified as usize != realized {
                return Err(Error::InvalidCharCount { specified, realized });
            }
        }
        if let Some(specified) = self.kerning_count {
            let realized = self.kernings.len();
            if specified as usize != realized {
                return Err(Error::InvalidKerningCount { specified, realized });
            }
        }
        let info = self.info.take().ok_or(Error::NoInfoBlock)?;
        let common = self.common.take().ok_or(Error::NoCommonBlock)?;
        Ok(Font::new(info, common, self.pages, self.chars, self.kernings))
    }

    pub fn set_info<'a, A>(&mut self, line: Option<usize>, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        if self.info.is_some() {
            Err(Error::DuplicateTag { line, tag: "info".to_owned() })
        } else {
            self.info = Some(Info::load(attributes)?);
            Ok(())
        }
    }

    pub fn set_common<'a, A>(
        &mut self,
        line: Option<usize>,
        attributes: &mut A,
    ) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        if self.common.is_some() {
            Err(Error::DuplicateTag { line, tag: "common".to_owned() })
        } else {
            self.common = Some(Common::load(attributes)?);
            Ok(())
        }
    }

    pub fn page<'a, A>(&mut self, _line: Option<usize>, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        let Page { id, file } = Page::load(attributes)?;
        if id as usize == self.pages.len() {
            self.pages.push(file);
            Ok(())
        } else {
            Err(crate::Error::BrokenPageList)
        }
    }

    pub fn char<'a, A>(&mut self, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        self.chars.push(Char::load(attributes)?);
        Ok(())
    }

    pub fn chars<'a, A>(&mut self, line: Option<usize>, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        if self.char_count.is_some() {
            Err(Error::DuplicateCharCount { line })
        } else {
            Count::load(attributes).map(|Count { count }| {
                self.char_count = Some(count);
                self.chars.reserve(count as usize - self.chars.len())
            })
        }
    }

    pub fn kernings<'a, A>(&mut self, line: Option<usize>, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        if self.kerning_count.is_some() {
            Err(Error::DuplicateKerningCount { line })
        } else {
            Count::load(attributes).map(|Count { count }| {
                self.kerning_count = Some(count);
                self.kernings.reserve(count as usize - self.kernings.len())
            })
        }
    }

    pub fn kerning<'a, A>(&mut self, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'a>,
    {
        self.kernings.push(Kerning::load(attributes)?);
        Ok(())
    }
}

impl Default for FontBuilder {
    fn default() -> Self {
        Self {
            info: Option::None,
            common: Option::None,
            pages: Vec::default(),
            chars: Vec::default(),
            char_count: None,
            kernings: Vec::default(),
            kerning_count: None,
        }
    }
}

pub trait Load: Sized {
    fn load<'a, A: Attributes<'a>>(attributes: &mut A) -> crate::Result<Self>;
}

macro_rules! implement_load {
    ($object:ty, $(($type:ty, $id:expr, $key:expr, $field:ident)),+) => {
        impl Load for $object {
            fn load<'a, A: Attributes<'a>>(attributes: &mut A) -> crate::Result<Self> {
                let mut block = Self::default();
                let mut bit_mask: u32 = 0x0000_0000;
                while let Some(Attribute { key, value, line }) = attributes.next_attribute()? {
                    match key {
                        $(
                            $key => {
                                let bit = 1 << $id;
                                if bit_mask & bit != 0 {
                                    let key = String::from_utf8_lossy($key).into();
                                    return Err(Error::DuplicateKey{ line, key });
                                }
                                bit_mask |= bit;
                                match <$type>::parse_bytes(&value) {
                                    Ok(v) => block.$field = v,
                                    Err(err) => {
                                        let err = err.to_string();
                                        let key = String::from_utf8_lossy($key).into();
                                        return Err(Error::Parse{ line, entity:key, err });
                                    }
                                }
                            },
                        )*
                        key => {
                            let key = String::from_utf8(key.into()).map_err(|e| crate::Error::Parse {
                                line,
                                entity: "key".to_owned(),
                                err: e.to_string(),
                            })?;
                            return Err(Error::InvalidKey { line, key })

                        },
                    };
                }
                return Ok(block);
            }
        }
    };
}

implement_load!(
    Char,
    (u32, 0x0, b"id", id),
    (u16, 0x1, b"x", x),
    (u16, 0x2, b"y", y),
    (u16, 0x3, b"width", width),
    (u16, 0x4, b"height", height),
    (i16, 0x5, b"xoffset", xoffset),
    (i16, 0x6, b"yoffset", yoffset),
    (i16, 0x8, b"xadvance", xadvance),
    (u8, 0x9, b"page", page),
    (Chnl, 0xA, b"chnl", chnl)
);

implement_load!(
    Common,
    (u16, 0x0, b"lineHeight", line_height),
    (u16, 0x1, b"base", base),
    (u16, 0x2, b"scaleW", scale_w),
    (u16, 0x3, b"scaleH", scale_h),
    (u16, 0x4, b"pages", pages),
    (bool, 0x5, b"packed", packed),
    (Packing, 0x6, b"alphaChnl", alpha_chnl),
    (Packing, 0x7, b"redChnl", red_chnl),
    (Packing, 0x8, b"greenChnl", green_chnl),
    (Packing, 0x9, b"blueChnl", blue_chnl)
);

implement_load!(Count, (u32, 0x0, b"count", count));

implement_load!(
    Info,
    (String, 0x0, b"face", face),
    (i16, 0x1, b"size", size),
    (bool, 0x2, b"bold", bold),
    (bool, 0x3, b"italic", italic),
    (Charset, 0x4, b"charset", charset),
    (bool, 0x5, b"unicode", unicode),
    (u16, 0x6, b"stretchH", stretch_h),
    (bool, 0x7, b"smooth", smooth),
    (u8, 0x8, b"aa", aa),
    (Padding, 0x9, b"padding", padding),
    (Spacing, 0xA, b"spacing", spacing),
    (u8, 0xB, b"outline", outline)
);

implement_load!(
    Kerning,
    (u32, 0x0, b"first", first),
    (u32, 0x1, b"second", second),
    (i16, 0x2, b"amount", amount)
);

implement_load!(Page, (u16, 0x0, b"id", id), (String, 0x1, b"file", file));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Count {
    count: u32,
}
