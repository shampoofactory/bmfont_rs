use crate::charset::Charset;
use crate::font::{Char, Chnl, Common, Info, Padding, Page, Spacing};
use crate::font::{Kerning, Packing};
use crate::parse::Parse;
use crate::Error;

use super::attributes::{Attribute, Attributes};
use super::Count;

pub trait Load: Sized {
    fn load<'b, A: Attributes<'b>>(attributes: &mut A) -> crate::Result<Self>;
}

macro_rules! implement_load {
    ($object:ty, $(($type:ty, $id:expr, $key:expr, $field:ident)),+) => {
        impl Load for $object {
            fn load<'b, A: Attributes<'b>>(attributes: &mut A) -> crate::Result<Self> {
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
