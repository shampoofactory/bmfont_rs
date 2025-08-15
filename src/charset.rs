#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::parse::{Parse, ParseResult};

use std::fmt;

/// ANSI character encoding.
pub const ANSI: u8 = 0;
/// Default character encoding.
pub const DEFAULT: u8 = 1;
/// Symbol character encoding.
pub const SYMBOL: u8 = 2;
/// Shift JIS character encoding.
pub const SHIFTJIS: u8 = 128;
/// Hangul character encoding.
pub const HANGUL: u8 = 129;
/// GB 2312 Chinese character encoding.
pub const GB2312: u8 = 134;
/// Big5 Chinese character encoding.
pub const CHINESEBIG5: u8 = 136;
/// OEM character encoding.
pub const OEM: u8 = 255;
/// Johab Korean character encoding.
pub const JOHAB: u8 = 130;
/// Hebrew character encoding.
pub const HEBREW: u8 = 177;
/// Arabic character encoding.
pub const ARABIC: u8 = 178;
/// Greek character encoding.
pub const GREEK: u8 = 161;
/// Turkish character encoding.
pub const TURKISH: u8 = 162;
/// Vietnamese character encoding.
pub const VIETNAMESE: u8 = 163;
/// Thai character encoding.
pub const THAI: u8 = 222;
/// Eastern Europe character encoding.
pub const EASTEUROPE: u8 = 238;
/// Russian character encoding.
pub const RUSSIAN: u8 = 204;
/// Max character encoding.
pub const MAC: u8 = 77;
/// Baltic character encoding.
pub const BALTIC: u8 = 186;

/// Non-Unicode character set encoding.
///
/// Defines the character set encoding when using non-Unicode fonts.
/// When Unicode is in use, that is [Info::unicode](crate::Info::unicode) is true, this should be set to [Charset::Null].
///
/// When manually constructing, use the [Charset::Tagged] variant with the required character set
/// constant (tag). See examples below.
///
/// The [Charset::Undefined] variant holds a non-canonical string value. It is constructed by
/// text-based format load methods when the string value is not a recognized tag. This undefined
/// string value is discarded when converting to a canonical format e.g. binary.
///
/// # Conversion rules: Binary
///
/// The binary format `Charset` field is composed of a single byte: `u8`.
///
/// `u8` to `Charset`:
/// - `0 if unicode  => Null`
/// - `u             => Tagged(u)`
///
/// `Charset` to `u8`:
/// - `Charset::Null | Charset::Undefined(_) => 0` (undefined value discarded)
/// - `Tagged(u)                             => u`
///
/// # Conversion rules: `String`
///
/// `String` to `Charset`, in order of precedence:
/// - `""  => Null`
/// - `TAG => Tagged(tag)`, where `TAG` is an defined character set and `tag` it's value
/// - `u   => Tagged(u)`, where `u` is a `u8` integer string
/// - `s   => Undefined(s)`, where `s` is any other string
///
/// e.g. `"HANGUL" => Tagged(129)`
///
/// `Charset` to `String`:
/// - `Null         => ""`
/// - `Tagged(tag)  => TAG`, where `TAG` is a defined character set and `tag` it's value
/// - `Tagged(u)    => u`, where `u` is a `u8` integer string
/// - `Undefined(s) => s`, where `s` is any other string (non-canonical)
///
/// e.g. `Tagged(163) => "VIETNAMESE"`
///
/// # Character set constants (tags)
///
/// - [ANSI]
/// - [ARABIC]
/// - [BALTIC]
/// - [CHINESEBIG5]
/// - [DEFAULT]
/// - [EASTEUROPE]
/// - [GB2312]
/// - [GREEK]
/// - [HANGUL]
/// - [HEBREW]
/// - [JOHAB]
/// - [MAC]
/// - [OEM]
/// - [RUSSIAN]
/// - [SHIFTJIS]
/// - [SYMBOL]
/// - [THAI]
/// - [TURKISH]
/// - [VIETNAMESE]
///
/// # Examples
///
/// ```
/// # use bmfont_rs::Charset;
/// # use bmfont_rs::ANSI;
/// // Construct ANSI
/// let charset = Charset::Tagged(ANSI);
/// ```
///
/// ```
/// # use bmfont_rs::Charset;
/// # use bmfont_rs::SHIFTJIS;
/// // Parse BMFont defined charset encoding constant string (case sensitive)
/// let name = "SHIFTJIS";
/// let charset: Charset = name.into();
/// assert_eq!(charset, Charset::Tagged(SHIFTJIS));
/// ```
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "String"),
    serde(into = "String")
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Charset {
    /// Used with Unicode character set encoding to indicate no other character set encoding
    /// is in use.
    Null,
    /// Used with BMFont defined character set encoding constants.
    Tagged(u8),
    /// Used with non-BMFont defined character set encodings.
    Undefined(String),
}

impl Default for Charset {
    #[inline(always)]
    fn default() -> Self {
        Self::Null
    }
}

impl fmt::Display for Charset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp: String;
        write!(
            f,
            "{}",
            match self {
                Self::Null => "",
                Self::Tagged(u) => match u {
                    0 => "ANSI",
                    1 => "DEFAULT",
                    2 => "SYMBOL",
                    77 => "MAC",
                    128 => "SHIFTJIS",
                    129 => "HANGUL",
                    130 => "JOHAB",
                    134 => "GB2312",
                    136 => "CHINESEBIG5",
                    161 => "GREEK",
                    162 => "TURKISH",
                    163 => "VIETNAMESE",
                    177 => "HEBREW",
                    178 => "ARABIC",
                    186 => "BALTIC",
                    204 => "RUSSIAN",
                    222 => "THAI",
                    238 => "EASTEUROPE",
                    255 => "OEM",
                    u => {
                        tmp = u.to_string();
                        &tmp
                    }
                },
                Self::Undefined(u) => u,
            }
        )
    }
}

impl From<&str> for Charset {
    fn from(s: &str) -> Charset {
        match s {
            "" => Charset::Null,
            "ANSI" => Charset::Tagged(ANSI),
            "ARABIC" => Charset::Tagged(ARABIC),
            "BALTIC" => Charset::Tagged(BALTIC),
            "CHINESEBIG5" => Charset::Tagged(CHINESEBIG5),
            "DEFAULT" => Charset::Tagged(DEFAULT),
            "EASTEUROPE" => Charset::Tagged(EASTEUROPE),
            "GB2312" => Charset::Tagged(GB2312),
            "GREEK" => Charset::Tagged(GREEK),
            "HANGUL" => Charset::Tagged(HANGUL),
            "HEBREW" => Charset::Tagged(HEBREW),
            "JOHAB" => Charset::Tagged(JOHAB),
            "MAC" => Charset::Tagged(MAC),
            "OEM" => Charset::Tagged(OEM),
            "RUSSIAN" => Charset::Tagged(RUSSIAN),
            "SHIFTJIS" => Charset::Tagged(SHIFTJIS),
            "SYMBOL" => Charset::Tagged(SYMBOL),
            "THAI" => Charset::Tagged(THAI),
            "TURKISH" => Charset::Tagged(TURKISH),
            "VIETNAMESE" => Charset::Tagged(VIETNAMESE),
            src => match src.parse::<u8>() {
                Ok(u) => Charset::Tagged(u),
                Err(_) => Charset::Undefined(src.to_owned()),
            },
        }
    }
}

impl From<String> for Charset {
    fn from(s: String) -> Charset {
        s.as_str().into()
    }
}

impl From<Charset> for String {
    fn from(v: Charset) -> Self {
        v.to_string()
    }
}

impl Parse for Charset {
    fn parse(src: &str) -> ParseResult<Self> {
        Ok(src.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_strings() {
        for u in 0..=255 {
            let charset = Charset::Tagged(u);
            let string = charset.to_string();
            assert_eq!(charset, string.as_str().into());
        }
    }

    #[test]
    fn from_str_null() {
        assert_eq!(Charset::Null, Charset::from(""));
    }

    #[test]
    fn from_str_tag() {
        assert_eq!(Charset::Tagged(HANGUL), Charset::from("HANGUL"));
    }

    #[test]
    fn from_str_u8() {
        assert_eq!(Charset::Tagged(HEBREW), Charset::from("177"));
    }

    #[test]
    fn from_str_undefined() {
        assert_eq!(Charset::Undefined("Unknown".to_owned()), Charset::from("Unknown"));
    }

    #[test]
    fn to_string_null() {
        assert_eq!("", Charset::Null.to_string());
    }

    #[test]
    fn to_string_tagged() {
        assert_eq!("GREEK", Charset::Tagged(GREEK).to_string());
    }

    #[test]
    fn to_string_u8() {
        assert_eq!("254", Charset::Tagged(254).to_string());
    }

    #[test]
    fn to_string_undefined() {
        assert_eq!("Unknown", Charset::Undefined("Unknown".to_owned()).to_string());
    }
}
