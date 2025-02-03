use std::fmt;
use std::io;

use crate::Charset;

/// Error Result.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate errors.
///
/// Describes the various errors that may occur when encoding/ decoding/ manipulating BMFont data
/// structures.
///
///
/// The list of variants may change over time. Other than [Error::Io] and [Error::Internal] you'll
/// probably not want to match against them.
///
///
/// `From<bmfont_rs::Error> for std::io::Error` is provided as a convenience.
///
///
/// The [Error::Internal] variant indicates malfunctioning library code and should be
/// reported at the project repository home
/// [here](https://github.com/shampoofactory/bmfont_rs/issues).
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The specified page ids do not form a coherent/ sequential list (decode only).
    BrokenPageList,
    /// Duplicate character count (decode only).
    DuplicateCharCount {
        /// Line where the error occurred.
        line: Option<usize>,
    },
    /// Duplicate character id.
    DuplicateChar {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Duplicate char id.
        id: u32,
    },
    /// Duplicate common block (decode only).
    DuplicateCommonBlock {
        /// Line where the error occurred.
        line: Option<usize>,
    },
    /// Duplicate info block (decode only).
    DuplicateInfoBlock {
        /// Line where the error occurred.
        line: Option<usize>,
    },
    /// Duplicate kerning count (decode only).
    DuplicateKerningCount {
        /// Line where the error occurred.
        line: Option<usize>,
    },
    /// Duplicate kerning pair entry.
    DuplicateKerningPair {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Kerning first character id.
        first: u32,
        /// Kerning second character id.
        second: u32,
    },
    /// Duplicate tagged key value (decode only).
    DuplicateKey {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Duplicate key.
        key: String,
    },
    /// Duplicate page id (decode only).
    DuplicatePageId {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Duplicate page id.
        id: u32,
    },
    /// Duplicate tag (decode only).
    DuplicateTag {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Duplicate tag.
        tag: String,
    },
    /// Page name lengths are not all of the same size.
    IncongruentPageNameLen {
        /// Line where the error occurred.
        line: Option<usize>,
    },
    /// The input is not a valid BMFont binary file (decode only).
    InvalidBinary {
        /// Magic bytes.
        magic_bytes: u32,
    },
    /// Invalid binary block (decode only).
    InvalidBinaryBlock {
        /// Block id.
        id: u8,
    },
    /// Invalid binary block length (decode only).
    InvalidBinaryEncoding {
        /// True if Unicode.
        unicode: bool,
        /// Character set encoding.
        charset: Charset,
    },
    /// The specified character count does not match the number of realized characters
    /// (decode only).
    InvalidCharCount {
        /// Specified count.
        specified: u32,
        /// Realized count.
        realized: usize,
    },
    /// The specified character page does not exist.
    InvalidCharPage {
        /// Character id.
        char_id: u32,
        /// Page id.
        page_id: u32,
    },
    /// The specified kerning pair count does not match the number of realized kerning pairs
    /// (decode only).
    InvalidKerningCount {
        /// Specified count.
        specified: u32,
        /// Realized count.
        realized: usize,
    },
    /// The specified kerning character does not exist.
    InvalidKerningChar {
        /// Character id.
        id: u32,
    },
    /// The tagged key name is not valid (decode only).
    InvalidKey {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Invalid key.
        key: String,
    },
    /// The specified page count does not match the number of realized pages
    /// (decode only).
    InvalidPageCount {
        /// Specified count.
        specified: u16,
        /// Realized count.
        realized: usize,
    },
    /// The tag name is not valid (decode only).
    InvalidTag {
        /// Line where the error occurred.
        line: Option<usize>,
        /// Invalid tag.
        tag: String,
    },
    /// The common block is missing.
    NoCommonBlock,
    /// The info block is missing.
    NoInfoBlock,
    /// There was an error parsing an entity.
    Parse {
        /// Line where the error occurred.
        line: Option<usize>,
        /// The entity that failed to parse.
        entity: String,
        /// The parse error.
        err: String,
    },
    /// The value string contains potentially unsafe control characters.
    UnsafeValueString {
        /// Path/ location.
        path: String,
        /// Value.
        value: String,
    },
    /// The binary version is unsupported (decode only).
    UnsupportedBinaryVersion {
        /// Binary version.
        version: u8,
    },
    /// The value string contains characters that cannot be encoded.
    UnsupportedValueEncoding {
        /// Path/ location.
        path: String,
        /// Value.
        value: String,
    },
    /// Internal error. This should not occur.
    Internal {
        /// Error.
        err: String,
    },
    /// Io error.
    Io {
        /// IO error
        err: io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BrokenPageList => {
                write!(f, "broken page list")
            }
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
            Error::DuplicatePageId { line, id } => {
                write!(f, "{}duplicate page id: {}", format_line(line), id)
            }
            Error::DuplicateTag { line, tag } => {
                write!(f, "{}duplicate tag: '{}'", format_line(line), tag)
            }
            Error::IncongruentPageNameLen { line } => {
                write!(f, "{}incongruent page file length", format_line(line))
            }
            Error::InvalidBinary { magic_bytes } => {
                write!(f, "invalid binary: magic bytes: {:08X}", magic_bytes)
            }
            Error::InvalidBinaryBlock { id } => {
                write!(f, "invalid binary block: id: {}", id)
            }
            Error::InvalidBinaryEncoding { unicode, charset } => {
                write!(f, "invalid binary encoding: unicode: {}, charset: {}", unicode, charset)
            }
            Error::InvalidCharCount { specified, realized } => {
                write!(f, "invalid char count: specified: {}, realized: {}", specified, realized)
            }
            Error::InvalidCharPage { char_id, page_id } => {
                write!(f, "invalid char page id: char id: {}, page id: {}", char_id, page_id)
            }
            Error::InvalidKerningCount { specified, realized } => {
                write!(f, "invalid kerning count: specified: {}, realized: {}", specified, realized)
            }
            Error::InvalidKerningChar { id } => {
                write!(f, "invalid kerning char: {}", id)
            }
            Error::InvalidKey { line, key } => {
                write!(f, "{}invalid key: '{}'", format_line(line), key)
            }
            Error::InvalidPageCount { specified, realized } => {
                write!(f, "invalid page count: specified: {}, realized: {}", specified, realized)
            }
            Error::InvalidTag { line, tag } => {
                write!(f, "{}invalid tag: '{}'", format_line(line), tag)
            }
            Error::NoCommonBlock => {
                write!(f, "no common block")
            }
            Error::NoInfoBlock => {
                write!(f, "no info block")
            }
            Error::Parse { line, entity, err } => {
                write!(f, "{}parse error: {}: {}", format_line(line), entity, err)
            }
            Error::UnsafeValueString { path, value } => {
                write!(f, "{}: unsafe value string: '{}'", path, value)
            }
            Error::UnsupportedBinaryVersion { version } => {
                write!(f, "unsupported version: {}", version)
            }
            Error::UnsupportedValueEncoding { path, value } => {
                write!(f, "{}: unsupported value encoding: '{}'", path, value)
            }
            Error::Internal { err } => {
                write!(f, "internal error: {}", err)
            }
            Error::Io { err } => {
                write!(f, "io: {}", err)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io { err }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Io { err } => err,
            err @ Error::Internal { .. } => io::Error::new(io::ErrorKind::Other, err),
            err @ Error::UnsupportedBinaryVersion { .. } => {
                io::Error::new(io::ErrorKind::Other, err)
            }
            err => io::Error::new(io::ErrorKind::InvalidData, err),
        }
    }
}

fn format_line(line: &Option<usize>) -> String {
    if let Some(line) = line {
        format!("line: {}: ", line)
    } else {
        "".to_owned()
    }
}
