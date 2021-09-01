use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

pub trait Parse: Sized {
    fn parse(src: &str) -> ParseResult<Self>;

    #[inline(always)]
    fn parse_bytes(bytes: &[u8]) -> ParseResult<Self> {
        Self::parse(std::str::from_utf8(bytes)?)
    }
}

impl<T: Copy + Default + Parse, const N: usize> Parse for [T; N] {
    fn parse(src: &str) -> ParseResult<Self> {
        let mut arr = [T::default(); N];
        let mut ts = src.split_terminator(",");
        for i in 0..N {
            if let Some(t) = ts.next() {
                arr[i] = T::parse(t.trim())?;
            } else {
                return Err(ParseError::ArrayUnderflow);
            }
        }
        if ts.next().is_some() {
            return Err(ParseError::ArrayOverflow);
        }
        Ok(arr)
    }
}

impl Parse for String {
    fn parse(src: &str) -> ParseResult<Self> {
        Self::parse_bytes(src.as_bytes())
    }

    fn parse_bytes(bytes: &[u8]) -> ParseResult<Self> {
        std::str::from_utf8(bytes).map(ToOwned::to_owned).map_err(Into::into)
    }
}

impl Parse for i16 {
    fn parse(src: &str) -> ParseResult<Self> {
        Self::from_str_radix(src, 10).map_err(Into::into)
    }
}

impl Parse for u32 {
    fn parse(src: &str) -> ParseResult<Self> {
        Self::from_str_radix(src, 10).map_err(Into::into)
    }
}

impl Parse for u16 {
    fn parse(src: &str) -> ParseResult<Self> {
        Self::from_str_radix(src, 10).map_err(Into::into)
    }
}

impl Parse for u8 {
    fn parse(src: &str) -> ParseResult<Self> {
        Self::from_str_radix(src, 10).map_err(Into::into)
    }
}

impl Parse for bool {
    fn parse(src: &str) -> ParseResult<Self> {
        u32::from_str_radix(src, 10).map(|u| u != 0).map_err(Into::into)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    ArrayUnderflow,
    ArrayOverflow,
    ParseIntError(ParseIntError),
    FromUtf8Error(FromUtf8Error),
    Utf8Error(Utf8Error),
    Other(String),
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ArrayUnderflow => write!(f, "array underflow"),
            ParseError::ArrayOverflow => write!(f, "array overflow"),
            ParseError::ParseIntError(err) => write!(f, "integer: {}", err),
            ParseError::FromUtf8Error(err) => write!(f, "UTF8: {}", err),
            ParseError::Utf8Error(err) => write!(f, "UTF8: {}", err),
            ParseError::Other(err) => write!(f, "{}", err),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(error: FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_4() {
        assert_eq!(<[u8; 4]>::parse("1,2,3,4"), Ok([1, 2, 3, 4]));
    }

    #[test]
    fn u8_4_ws() {
        assert_eq!(<[u8; 4]>::parse(" 1 , 2 , 3 , 4 "), Ok([1, 2, 3, 4]));
    }

    #[test]
    fn u8_4_underflow() {
        assert_eq!(<[u8; 4]>::parse("1,2,3"), Err(ParseError::ArrayUnderflow));
    }

    #[test]
    fn u8_4_overflow() {
        assert_eq!(<[u8; 4]>::parse("1,2,3,4,5"), Err(ParseError::ArrayOverflow));
    }
}
