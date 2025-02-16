use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::parse::{Parse, ParseError, ParseResult};

use super::charset::Charset;

/// Bitmap font descriptor.
///
/// This object holds, in it's entirety, the information contained within a BMFont descriptor file.
/// This, when paired with the associated texture file/s, allows us to render the described font.
///
/// Data is structured in accordance with the
/// [Bitmap Font Generator - Documentation](http://www.angelcode.com/products/bmfont/doc/file_format.html)
/// .
///
/// N.B. Certain tools deviate from the BMFont standard and can generate incompatible files.
///
/// Outline:
///
/// - `info`: holds information on how the font was generated.
/// - `common`: holds information common to all characters.
/// - `pages`: holds an ordered list of texture files, the index corresponds to the page id.
/// - `chars`: holds an unordered list of character descriptions.
/// - `kernings` holds an unordered list of kerning pairs.
///
/// For efficient usage you'll likely want to convert `chars` and `kernings` to maps.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Font {
    /// Font information.
    pub info: Info,
    /// Common character description.
    pub common: Common,
    /// Texture filenames.
    pub pages: Vec<String>,
    /// Character descriptors.
    pub chars: Vec<Char>,
    /// Kerning pairs.
    pub kernings: Vec<Kerning>,
}

impl Font {
    /// Construct a new Font.
    ///
    /// N.B. The supplied arguments are not validated.
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

    /// Validate references. Ensure that all page/ character references exist. In other words, that
    /// we don't have references to a non-existent page/ character.
    pub fn validate_references(&self) -> crate::Result<()> {
        self.validate_char_references()?;
        self.validate_kerning_references()?;
        Ok(())
    }

    fn validate_char_references(&self) -> crate::Result<()> {
        for char in &self.chars {
            if self.pages.len() <= char.page as usize {
                return Err(crate::Error::InvalidCharPage {
                    char_id: char.id,
                    page_id: char.page as u32,
                });
            }
        }
        Ok(())
    }

    fn validate_kerning_references(&self) -> crate::Result<()> {
        let set: HashSet<u32> = self.chars.iter().map(|u| u.id).collect();
        for kerning in &self.kernings {
            if !set.contains(&kerning.first) {
                return Err(crate::Error::InvalidKerningChar { id: kerning.first });
            }
            if !set.contains(&kerning.second) {
                return Err(crate::Error::InvalidKerningChar { id: kerning.second });
            }
        }
        Ok(())
    }
}

/// Character description.
///
/// This block describes a character in the font.
/// There is one for each included character in the font.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Char {
    /// The character id.
    pub id: u32,
    /// The left position of the character image in the texture.
    pub x: u16,
    /// The top position of the character image in the texture.   
    pub y: u16,
    /// The width of the character image in the texture.
    pub width: u16,
    /// The height of the character image in the texture.
    pub height: u16,
    /// How much the current position should be offset when copying the image from the texture to
    /// the screen.
    pub xoffset: i16,
    /// How much the current position should be offset when copying the image from the texture to
    /// the screen.
    pub yoffset: i16,
    /// How much the current position should be advanced after drawing the character.
    pub xadvance: i16,
    /// The texture page where the character image is found.
    pub page: u8,
    /// The texture channel where the character image is found.
    pub chnl: Chnl,
}

impl Char {
    /// Construct a new Char.
    ///
    /// N.B. The supplied arguments are not validated.
    #[allow(clippy::too_many_arguments)]
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

/// Common character description.
///
/// This block holds information common to all characters.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Common {
    /// This is the distance in pixels between each line of text.
    pub line_height: u16,
    /// The number of pixels from the absolute top of the line to the base of the characters.
    pub base: u16,
    /// The width of the texture, normally used to scale the x pos of the character image.
    pub scale_w: u16,
    /// The height of the texture, normally used to scale the y pos of the character image.
    pub scale_h: u16,
    /// The number of texture pages included in the font.
    pub pages: u16,
    /// True if the monochrome characters have been packed into each of the texture channels.
    /// In this case the channel packing describes what is stored in each channel.
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub packed: bool,
    /// Alpha channel packing.
    pub alpha_chnl: Packing,
    /// Red channel packing.
    pub red_chnl: Packing,
    /// Green channel packing.
    pub green_chnl: Packing,
    /// Blue channel packing.
    pub blue_chnl: Packing,
}

impl Common {
    /// Construct a new Common block.
    ///
    /// N.B. The supplied arguments are not validated.
    #[allow(clippy::too_many_arguments)]
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

/// Font information.
///
/// This block holds information on how the font was generated.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Info {
    /// This is the name of the true type font.
    pub face: String,
    /// The size of the true type font.
    pub size: i16,
    /// True if the font is bold.
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub bold: bool,
    /// True if the font is italic.
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub italic: bool,
    /// The name of the OEM charset (when not Unicode).
    pub charset: Charset,
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    /// True if Unicode charset.
    pub unicode: bool,
    /// The font height stretch in percentage. 100% means no stretch.
    pub stretch_h: u16,
    /// True if smoothing was turned on.
    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "se_bool"),
        serde(deserialize_with = "de_bool")
    )]
    pub smooth: bool,
    /// The supersampling level used. 1 means no supersampling was used.
    pub aa: u8,
    /// The padding for each character.
    pub padding: Padding,
    /// The spacing for each character.
    pub spacing: Spacing,
    /// The outline thickness for the characters.
    #[cfg_attr(feature = "serde", serde(default))]
    pub outline: u8,
}

impl Info {
    /// Construct a new Info block.
    ///
    /// N.B. The supplied arguments are not validated.
    #[allow(clippy::too_many_arguments)]
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

    #[allow(dead_code)]
    pub(crate) fn check_encoding(&self) -> crate::Result<()> {
        if self.unicode && self.charset != Charset::Null {
            return Err(crate::Error::InvalidCharsetEncoding {
                unicode: true,
                charset: self.charset.clone(),
            });
        }
        Ok(())
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            face: Default::default(),
            size: Default::default(),
            bold: Default::default(),
            italic: Default::default(),
            charset: Charset::Tagged(0),
            unicode: Default::default(),
            stretch_h: Default::default(),
            smooth: Default::default(),
            aa: Default::default(),
            padding: Default::default(),
            spacing: Default::default(),
            outline: Default::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
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

/// Character padding.
///
/// The padding for each character (up, right, down, left).
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "[u8; 4]"),
    serde(into = "[u8; 4]")
)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub up: u8,
    pub right: u8,
    pub down: u8,
    pub left: u8,
}

impl Padding {
    /// Construct a new Padding.
    ///
    /// N.B. The supplied arguments are not validated.
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

/// Character spacing.
///
/// The spacing for each character (horizontal, vertical).
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "[u8; 2]"),
    serde(into = "[u8; 2]")
)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Spacing {
    pub horizontal: u8,
    pub vertical: u8,
}

impl Spacing {
    /// Construct a new Spacing.
    ///
    /// N.B. The supplied arguments are not validated.
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

/// Kerning pair.
///
/// The kerning information is used to adjust the distance between certain characters,
/// e.g. some characters should be placed closer to each other than others.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Kerning {
    /// The first character id.
    pub first: u32,
    /// The second character id.
    pub second: u32,
    /// How much the x position should be adjusted when drawing the second character immediately
    /// following the first.
    pub amount: i16,
}

impl Kerning {
    /// Construct a new Kerning.
    ///
    /// N.B. The supplied arguments are not validated.
    #[inline(always)]
    pub fn new(first: u32, second: u32, amount: i16) -> Self {
        Self { first, second, amount }
    }
}

/// Channel packing description.
///
/// Used when character packing is specified to describe what is stored in each texture
/// channel.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "u8"),
    serde(into = "u8")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Packing {
    /// Channel holds glyph data.
    Glyph = 0,
    /// Channel holds outline data.
    Outline = 1,
    /// Channel holds glyph and outline data.
    GlyphOutline = 2,
    /// Channel is set to zero.
    Zero = 3,
    /// Channel is set to one.
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

/// Character texture channel description.
///
/// The texture channel/s where the character image is found.
///
/// That the official BMFont documentations only specifies five possible combinations:
/// red, blue, green, alpha and all.
///
/// These are encoded as the constants:
/// [RED](Self::RED),
/// [GREEN](Self::GREEN),
/// [BLUE](Self::BLUE),
/// [ALPHA](Self::ALPHA),
/// [ALL](Self::ALL),
///
///
/// Internally the structure is represented by a byte bit field. The individual channel bits can
/// be queried and set as desired. Unless you know what you're doing, take care when setting bits
/// to avoid non-standard combinations.
///
/// # Examples
///
/// ```
/// # use bmfont_rs::Chnl;
/// // Constructing using the standard constants
/// let chnl = Chnl::RED;
/// assert!(chnl.red());
/// assert!(!chnl.green());
/// assert!(!chnl.blue());
/// assert!(!chnl.alpha());
/// ```
///
/// ```
/// # use bmfont_rs::Chnl;
/// // Matching using the standard constants, although you'll likely want to throw an error
/// // rather than panic.
/// let chnl = Chnl::RED;
/// match chnl {
///     Chnl::RED => { /* RED handling code here */},
///     Chnl::ALL => { /* ALL handling code here */},
///     _ => { /* cannot handle */ panic!() }
/// }
/// ```
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(try_from = "u8"),
    serde(into = "u8")
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chnl(u8);

impl Chnl {
    /// Character image data is stored in all channels.    
    pub const ALL: Chnl = Chnl(15);

    /// Character image data is stored in the alpha channel.    
    pub const ALPHA: Chnl = Chnl(8);

    /// Character image data is stored in the red channel.    
    pub const RED: Chnl = Chnl(4);

    /// Character image data is stored in the green channel.    
    pub const GREEN: Chnl = Chnl(2);

    /// Character image data is stored in the blue channel.    
    pub const BLUE: Chnl = Chnl(1);

    /// The alpha channel bit.
    #[inline(always)]
    pub fn alpha(self) -> bool {
        self.0 & 8 != 0
    }

    /// Set the alpha channel bit.
    #[inline(always)]
    pub fn set_alpha(&mut self, v: bool) {
        if v {
            self.0 |= 8;
        } else {
            self.0 &= !8;
        }
    }

    /// The red channel bit.
    #[inline(always)]
    pub fn red(self) -> bool {
        self.0 & 4 != 0
    }

    /// Set the red channel bit.
    #[inline(always)]
    pub fn set_red(&mut self, v: bool) {
        if v {
            self.0 |= 4;
        } else {
            self.0 &= !4;
        }
    }

    /// The green channel bit.
    #[inline(always)]
    pub fn green(self) -> bool {
        self.0 & 2 != 0
    }

    /// Set the green channel bit.
    #[inline(always)]
    pub fn set_green(&mut self, v: bool) {
        if v {
            self.0 |= 2;
        } else {
            self.0 &= !2;
        }
    }

    /// The blue channel bit.
    #[inline(always)]
    pub fn blue(self) -> bool {
        self.0 & 1 != 0
    }

    /// Set the blue channel bit.
    #[inline(always)]
    pub fn set_blue(&mut self, v: bool) {
        if v {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }
}

impl Default for Chnl {
    #[inline(always)]
    fn default() -> Self {
        Self::ALL
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

#[cfg(feature = "serde")]
pub fn se_bool<S: Serializer>(v: &bool, s: S) -> Result<S::Ok, S::Error> {
    (*v as u8).serialize(s)
}

#[cfg(feature = "serde")]
pub fn de_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(u8::deserialize(d)? != 0)
}
