use std::fmt;
use std::io;

use crate::Charset;

pub type Result<T> = std::result::Result<T, Error>;

/// BMFont errors.
///
/// Describes the various errors that may occur when encoding/ decoding/ manipulating BMFont data
/// structures.
///
/// The [Internal](Error::Internal) variant indicates malfunctioning crate code and should be
/// reported at the project repository home [here](https://github.com/shampoofactory/lzfse_rust/issues).
#[derive(Debug)]
pub enum Error {
    /// Duplicate character count (decode only).
    DuplicateCharCount { line: Option<usize> },
    /// Duplicate character id.
    DuplicateChar { line: Option<usize>, id: u32 },
    /// Duplicate common block (decode only).
    DuplicateCommonBlock { line: Option<usize> },
    /// Duplicate info block (decode only).
    DuplicateInfoBlock { line: Option<usize> },
    /// Duplicate kerning count (decode only).
    DuplicateKerningCount { line: Option<usize> },
    /// Duplicate kerning pair entry.
    DuplicateKerningPair { line: Option<usize>, first: u32, second: u32 },
    /// Duplicate tagged key value (decode only).
    DuplicateKey { line: Option<usize>, key: String },
    /// Duplicate page id (decode only).
    DuplicatePage { line: Option<usize>, id: u32 },
    /// Duplicate tag (decode only).
    DuplicateTag { line: Option<usize>, tag: String },
    /// Not all the specified page file names are equal, as stated in the BMFont binary
    /// specification.
    IncongruentPageFileLen { line: Option<usize> },
    /// The input is not a valid BMFont binary file (decode only).
    InvalidBinary { magic_bytes: u32 },
    /// Invalid binary block (decode only).
    InvalidBinaryBlock { id: u8 },
    /// Invalid binary block length (decode only).
    InvalidBinaryBlockLen { id: u8, len: u32 },
    /// Invalid binary block character set encoding.
    InvalidBinaryEncoding { unicode: bool, charset: Charset },
    /// Invalid binary version.
    InvalidBinaryVersion { version: u8 },
    /// The specified character count does not match the number of specified characters
    /// (decode only).
    InvalidCharCount { specified: u32, realized: usize },
    /// The specified kerning pair count does not match the number of specified kerning pairs
    /// (decode only).
    InvalidKerningCount { specified: u32, realized: usize },
    /// The tagged key name is not valid (decode only).
    InvalidKey { line: Option<usize>, key: String },
    /// The tag name is not valid (decode only).
    InvalidTag { line: Option<usize>, tag: String },
    /// The tagged key value is not valid (decode only).
    InvalidValue { line: Option<usize>, key: String, err: String },
    /// There is a gap in the list of specified page ids (decode only).
    MissingPageId { line: Option<usize>, id: u32 },
    /// The common block is missing.
    NoCommonBlock,
    /// The info block is missing.
    NoInfoBlock,
    /// There was an error parsing a value.
    Parse { line: Option<usize>, err: String },
    /// The binary version is unsupported (decode only).
    UnsupportedBinaryVersion { version: u8 },
    /// Internal error. This should not occur.
    Internal { err: String },
    /// Io error.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DuplicateCharCount { line } => {
                write!(f, "{}duplicate char count", format_line(line))
            }
            Error::DuplicateChar { line, id } => {
                write!(f, "{}duplicate char id: {}", format_line(line), id)
            }
            Error::DuplicateCommonBlock { line } => {
                write!(f, "{}duplicate common block", format_line(line))
            }
            Error::DuplicateInfoBlock { line } => {
                write!(f, "{}duplicate info block", format_line(line))
            }
            Error::DuplicateKerningCount { line } => {
                write!(f, "{}duplicate kerning count", format_line(line))
            }
            Error::DuplicateKerningPair { line, first, second } => {
                write!(f, "{}duplicate kerning pair: {}/ {}", format_line(line), first, second)
            }
            Error::DuplicateKey { line, key } => {
                write!(f, "{}duplicate key: '{}'", format_line(line), key)
            }
            Error::DuplicatePage { line, id } => {
                write!(f, "{}duplicate page id: {}", format_line(line), id)
            }
            Error::DuplicateTag { line, tag } => {
                write!(f, "{}duplicate tag: '{}'", format_line(line), tag)
            }
            Error::IncongruentPageFileLen { line } => {
                write!(f, "{}incongruent page file length", format_line(line))
            }
            Error::InvalidBinary { magic_bytes } => {
                write!(f, "invalid binary: magic bytes: {:08X}", magic_bytes)
            }
            Error::InvalidBinaryBlock { id } => {
                write!(f, "invalid binary block: id: {}", id)
            }
            Error::InvalidBinaryBlockLen { id, len } => {
                write!(f, "invalid binary block: id: {}, len: {}", id, len)
            }
            Error::InvalidBinaryEncoding { unicode, charset } => {
                write!(f, "invalid binary encoding: unicode: {}, charset: {}", unicode, charset)
            }
            Error::InvalidBinaryVersion { version } => {
                write!(f, "invalid binary version: {}", version)
            }
            Error::InvalidCharCount { specified, realized } => {
                write!(f, "invalid char count: specified: {}, realized: {}", specified, realized)
            }
            Error::InvalidKerningCount { specified, realized } => {
                write!(f, "invalid kerning count: specified: {}, realized: {}", specified, realized)
            }
            Error::InvalidKey { line, key } => {
                write!(f, "{}invalid key: '{}'", format_line(line), key)
            }
            Error::InvalidTag { line, tag } => {
                write!(f, "{}invalid tag: '{}'", format_line(line), tag)
            }
            Error::InvalidValue { line, key, err } => {
                write!(f, "{}invalid value: '{}', {}", format_line(line), key, err)
            }
            Error::MissingPageId { line, id } => {
                write!(f, "{}missing page id: {}", format_line(line), id)
            }
            Error::NoCommonBlock => {
                write!(f, "no common block")
            }
            Error::NoInfoBlock => {
                write!(f, "no info block")
            }
            Error::Parse { line, err } => {
                write!(f, "{}parse error: {}", format_line(line), err)
            }
            Error::UnsupportedBinaryVersion { version } => {
                write!(f, "unsupported version: {}", version)
            }
            Error::Internal { err } => {
                write!(f, "internal error: {}", err)
            }
            Error::Io(err) => {
                write!(f, "io: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

fn format_line(line: &Option<usize>) -> String {
    if let Some(line) = line {
        format!("{}", line)
    } else {
        "".to_owned()
    }
}
