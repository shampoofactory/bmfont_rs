#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::parse::{Parse, ParseResult};

use std::fmt;

pub const ANSI: u8 = 0;
pub const DEFAULT: u8 = 1;
pub const SYMBOL: u8 = 2;
pub const SHIFTJIS: u8 = 128;
pub const HANGUL: u8 = 129;
pub const GB2312: u8 = 134;
pub const CHINESEBIG5: u8 = 136;
pub const OEM: u8 = 255;
pub const JOHAB: u8 = 130;
pub const HEBREW: u8 = 177;
pub const ARABIC: u8 = 178;
pub const GREEK: u8 = 161;
pub const TURKISH: u8 = 162;
pub const VIETNAMESE: u8 = 163;
pub const THAI: u8 = 222;
pub const EASTEUROPE: u8 = 238;
pub const RUSSIAN: u8 = 204;
pub const MAC: u8 = 77;
pub const BALTIC: u8 = 186;

/// Non-Unicode character set encoding.
///
/// When manually constructing, prefer constructing the `Tagged` variant.
/// This ensures unambiguous binary format conversion.
///
/// ## Conversion rules: Binary
///
/// The binary format `Charset` field is composed of a single byte: `u8`.
///
/// `u8` to `Charset`:
/// - `0 => Null | Tagged(0) | Undefined`, context sensitive
/// - `u => Tagged(u)`
///
/// `Charset` to `u8`:
/// - `Null => 0`,
/// - `Tagged(u) => u`
/// - `Undefined => 0`
///
/// ## Conversion rules: `String`
///
/// `String` to `Charset`, in order of precedence:
/// - `"" => Null`
/// - `TAG => Tagged(tag)`, where `TAG` is a defined tag label and `tag` it's value
/// - `u_string => Tagged(u)`, where `u_string` is a base-10 representation of `u`
/// - `_  => Undefined(_)`, where `_` is any `String`
///
/// e.g. `"HANGUL" => Tagged(129)`
///
/// `Charset` to `String`:
/// - `Null => ""`
/// - `Tagged(tag) => TAG`, where `TAG` is a defined tag label and `tag` it's value  
/// - `Tagged(u) => u_string`, where `u_string` is a base-10 representation of `u`  
/// - `Undefined(undefined) => undefined`
///
/// e.g. `Tagged(163) => "VIETNAMESE"`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(from = "&str"),
    serde(into = "String")
)]
pub enum Charset {
    Null,
    Tagged(u8),
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
                    128 => "SHIFTJIS",
                    129 => "HANGUL",
                    134 => "GB2312",
                    136 => "CHINESEBIG5",
                    130 => "JOHAB",
                    177 => "HEBREW",
                    178 => "ARABIC",
                    161 => "GREEK",
                    162 => "TURKISH",
                    163 => "VIETNAMESE",
                    222 => "THAI",
                    238 => "EASTEUROPE",
                    204 => "RUSSIAN",
                    77 => "MAC",
                    186 => "BALTIC",
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
            "ANSI" => Charset::Tagged(0),
            "DEFAULT" => Charset::Tagged(1),
            "SYMBOL" => Charset::Tagged(2),
            "SHIFTJIS" => Charset::Tagged(128),
            "HANGUL" => Charset::Tagged(129),
            "GB2312" => Charset::Tagged(134),
            "CHINESEBIG5" => Charset::Tagged(136),
            "OEM" => Charset::Tagged(255),
            "JOHAB" => Charset::Tagged(130),
            "HEBREW" => Charset::Tagged(177),
            "ARABIC" => Charset::Tagged(178),
            "GREEK" => Charset::Tagged(161),
            "TURKISH" => Charset::Tagged(162),
            "VIETNAMESE" => Charset::Tagged(163),
            "THAI" => Charset::Tagged(222),
            "EASTEUROPE" => Charset::Tagged(238),
            "RUSSIAN" => Charset::Tagged(204),
            "MAC" => Charset::Tagged(77),
            "BALTIC" => Charset::Tagged(186),
            src => match u8::from_str_radix(src, 10) {
                Ok(u) => Charset::Tagged(u),
                Err(_) => Charset::Undefined(src.to_owned()),
            },
        }
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
