use std::convert::{TryFrom, TryInto};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::parse::{Parse, ParseError, ParseResult};

use super::charset::Charset;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Font {
    pub info: Info,
    pub common: Common,
    pub pages: Vec<String>,
    pub chars: Vec<Char>,
    pub kernings: Vec<Kerning>,
}

impl Font {
    #[inline(always)]
    pub fn new(
        info: Info,
        common: Common,
        pages: Vec<String>,
        chars: Vec<Char>,
        kernings: Vec<Kerning>,
    ) -> Self {
        Self { info, common, pages, chars, kernings }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Char {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub xoffset: i16,
    pub yoffset: i16,
    pub xadvance: i16,
    pub page: u8,
    pub chnl: Chnl,
}

impl Char {
    #[inline(always)]
    pub fn new(
        id: u32,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        xoffset: i16,
        yoffset: i16,
        xadvance: i16,
        page: u8,
        chnl: Chnl,
    ) -> Self {
        Self { id, x, y, width, height, xoffset, yoffset, xadvance, page, chnl }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Common {
    pub line_height: u16,
    pub base: u16,
    pub scale_w: u16,
    pub scale_h: u16,
    pub pages: u16,
    #[cfg_attr(
        all(feature = "serde", feature = "serde_boolint"),
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub packed: bool,
    pub alpha_chnl: Packing,
    pub red_chnl: Packing,
    pub green_chnl: Packing,
    pub blue_chnl: Packing,
}

impl Common {
    #[inline(always)]
    pub fn new(
        line_height: u16,
        base: u16,
        scale_w: u16,
        scale_h: u16,
        pages: u16,
        packed: bool,
        alpha_chnl: Packing,
        red_chnl: Packing,
        green_chnl: Packing,
        blue_chnl: Packing,
    ) -> Self {
        Self {
            line_height,
            base,
            scale_w,
            scale_h,
            pages,
            packed,
            alpha_chnl,
            red_chnl,
            green_chnl,
            blue_chnl,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct Info {
    pub face: String,
    pub size: i16,
    #[cfg_attr(
        all(feature = "serde", feature = "serde_boolint"),
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub bold: bool,
    #[cfg_attr(
        all(feature = "serde", feature = "serde_boolint"),
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub italic: bool,
    pub charset: Charset,
    #[cfg_attr(
        all(feature = "serde", feature = "serde_boolint"),
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub unicode: bool,
    pub stretch_h: u16,
    #[cfg_attr(
        all(feature = "serde", feature = "serde_boolint"),
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub smooth: bool,
    pub aa: u8,
    pub padding: Padding,
    pub spacing: Spacing,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outline: u8,
}

impl Info {
    #[inline(always)]
    pub fn new(
        face: String,
        size: i16,
        bold: bool,
        italic: bool,
        charset: Charset,
        unicode: bool,
        stretch_height: u16,
        smooth: bool,
        aa: u8,
        padding: Padding,
        spacing: Spacing,
        outline: u8,
    ) -> Self {
        Self {
            face,
            size,
            bold,
            italic,
            charset,
            unicode,
            stretch_h: stretch_height,
            smooth,
            aa,
            padding,
            spacing,
            outline,
        }
    }

    pub fn check_encoding(&self) -> crate::Result<()> {
        if self.unicode && self.charset != Charset::Null {
            return Err(crate::Error::InvalidBinaryEncoding {
                unicode: true,
                charset: self.charset.clone(),
            });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Page {
    pub id: u16,
    pub file: String,
}

// impl Page {
//     #[inline(always)]
//     pub fn new(id: u16, file: String) -> Self {
//         Self { id, file }
//     }
// }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "[u8; 4]"),
    serde(into = "[u8; 4]")
)]
pub struct Padding {
    pub up: u8,
    pub right: u8,
    pub down: u8,
    pub left: u8,
}

impl Padding {
    #[inline(always)]
    pub fn new(up: u8, right: u8, down: u8, left: u8) -> Self {
        Self { up, right, down, left }
    }
}

impl Parse for Padding {
    fn parse(src: &str) -> ParseResult<Self> {
        <[u8; 4]>::parse(src).map(Into::into)
    }

    fn parse_bytes(bytes: &[u8]) -> ParseResult<Self> {
        <[u8; 4]>::parse_bytes(bytes).map(Into::into)
    }
}

impl From<[u8; 4]> for Padding {
    #[inline(always)]
    fn from(array: [u8; 4]) -> Self {
        Self { up: array[0], right: array[1], down: array[2], left: array[3] }
    }
}

impl From<Padding> for [u8; 4] {
    #[inline(always)]
    fn from(padding: Padding) -> Self {
        [padding.up, padding.right, padding.down, padding.left]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "[u8; 2]"),
    serde(into = "[u8; 2]")
)]
pub struct Spacing {
    pub horizontal: u8,
    pub vertical: u8,
}

impl Spacing {
    #[inline(always)]
    pub fn new(horizontal: u8, vertical: u8) -> Self {
        Self { horizontal, vertical }
    }
}

impl Parse for Spacing {
    fn parse(src: &str) -> ParseResult<Self> {
        <[u8; 2]>::parse(src).map(Into::into)
    }

    fn parse_bytes(bytes: &[u8]) -> ParseResult<Self> {
        <[u8; 2]>::parse_bytes(bytes).map(Into::into)
    }
}

impl From<[u8; 2]> for Spacing {
    #[inline(always)]
    fn from(array: [u8; 2]) -> Self {
        Self { horizontal: array[0], vertical: array[1] }
    }
}

impl From<Spacing> for [u8; 2] {
    #[inline(always)]
    fn from(spacing: Spacing) -> Self {
        [spacing.horizontal, spacing.vertical]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kerning {
    pub first: u32,
    pub second: u32,
    pub amount: i16,
}

impl Kerning {
    #[inline(always)]
    pub fn new(first: u32, second: u32, amount: i16) -> Self {
        Self { first, second, amount }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "u8"),
    serde(into = "u8")
)]
#[repr(u8)]
pub enum Packing {
    Glyph = 0,
    Outline = 1,
    GlyphOutline = 2,
    Zero = 3,
    One = 4,
}

impl Default for Packing {
    #[inline(always)]
    fn default() -> Self {
        Self::Glyph
    }
}

impl From<Packing> for u8 {
    #[inline(always)]
    fn from(chnl: Packing) -> Self {
        chnl as u8
    }
}

impl TryFrom<u8> for Packing {
    type Error = ParseError;

    fn try_from(chnl: u8) -> Result<Self, Self::Error> {
        match chnl {
            0 => Ok(Self::Glyph),
            1 => Ok(Self::Outline),
            2 => Ok(Self::GlyphOutline),
            3 => Ok(Self::Zero),
            4 => Ok(Self::One),
            u => Err(ParseError::Other(format!("Packing: invalid u8: {}", u))),
        }
    }
}

impl Parse for Packing {
    fn parse(src: &str) -> ParseResult<Self> {
        let u: u8 = src.parse()?;
        let packing: Packing = u.try_into()?;
        Ok(packing)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "u8"),
    serde(into = "u8")
)]
pub struct Chnl(u8);

impl Chnl {
    pub const ALL: Chnl = Chnl(15);
    pub const ALPHA: Chnl = Chnl(8);
    pub const RED: Chnl = Chnl(4);
    pub const GREEN: Chnl = Chnl(2);
    pub const BLUE: Chnl = Chnl(1);
    pub const NONE: Chnl = Chnl(0);

    #[inline(always)]
    pub fn alpha(self) -> bool {
        self.0 & 8 != 0
    }

    #[inline(always)]
    pub fn set_alpha(&mut self, v: bool) {
        if v {
            self.0 |= 8;
        } else {
            self.0 &= !8;
        }
    }

    #[inline(always)]
    pub fn red(self) -> bool {
        self.0 & 4 != 0
    }

    #[inline(always)]
    pub fn set_red(&mut self, v: bool) {
        if v {
            self.0 |= 4;
        } else {
            self.0 &= !4;
        }
    }

    #[inline(always)]
    pub fn green(self) -> bool {
        self.0 & 2 != 0
    }

    #[inline(always)]
    pub fn set_green(&mut self, v: bool) {
        if v {
            self.0 |= 2;
        } else {
            self.0 &= !2;
        }
    }

    #[inline(always)]
    pub fn blue(self) -> bool {
        self.0 & 1 != 0
    }

    #[inline(always)]
    pub fn set_blue(&mut self, v: bool) {
        if v {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }
}

impl From<Chnl> for u8 {
    #[inline(always)]
    fn from(chnl: Chnl) -> Self {
        chnl.0
    }
}

impl TryFrom<u8> for Chnl {
    type Error = ParseError;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u < 0x10 {
            Ok(Self(u))
        } else {
            Err(ParseError::Other(format!("Chnl: invalid u8: {}", u)))
        }
    }
}

impl Parse for Chnl {
    fn parse(src: &str) -> ParseResult<Self> {
        let u: u8 = src.parse()?;
        let packing: Chnl = u.try_into()?;
        Ok(packing)
    }
}

#[cfg(all(feature = "serde", feature = "serde_boolint"))]
pub fn se_bool<S: Serializer>(v: &bool, s: S) -> Result<S::Ok, S::Error> {
    (*v as u8).serialize(s)
}

#[cfg(all(feature = "serde", feature = "serde_boolint"))]
pub fn de_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(u8::deserialize(d)? != 0)
}
