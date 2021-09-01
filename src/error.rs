use std::fmt;
use std::io;

use crate::Charset;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DuplicateCharCount { line: Option<usize> },
    DuplicateChar { line: Option<usize>, id: u32 },
    DuplicateCommonBlock { line: Option<usize> },
    DuplicateInfoBlock { line: Option<usize> },
    DuplicateKerningCount { line: Option<usize> },
    DuplicateKerningPair { line: Option<usize>, first: u32, second: u32 },
    DuplicateKey { line: Option<usize>, key: String },
    DuplicatePage { line: Option<usize>, id: u32 },
    DuplicateTag { line: Option<usize>, tag: String },
    IncongruentPageFileLen { line: Option<usize> },
    InvalidBinary { magic_bytes: u32 },
    InvalidBinaryBlock { id: u8 },
    InvalidBinaryBlockLen { id: u8, len: u32 },
    InvalidBinaryEncoding { unicode: bool, charset: Charset },
    InvalidBinaryVersion { version: u8 },
    InvalidCharCount { specified: u32, realized: usize },
    InvalidKerningCount { specified: u32, realized: usize },
    InvalidKey { line: Option<usize>, key: String },
    InvalidTag { line: Option<usize>, tag: String },
    InvalidValue { line: Option<usize>, key: String, err: String },
    MissingPageId { line: Option<usize>, id: u32 },
    NoCommonBlock,
    NoInfoBlock,
    Parse { line: Option<usize>, err: String },
    UnsupportedBinaryVersion { version: u8 },
    Internal { err: String },
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
