use std::fmt;

const CR: u8 = '\r' as u8;
const EQ: u8 = '=' as u8;
const LF: u8 = '\n' as u8;
const QT: u8 = '"' as u8;
const SP: u8 = ' ' as u8;
const TB: u8 = '\t' as u8;

/// Tagged attribute parser.
///
/// Data is encoded as lines that comprise tagged attributes.
///
/// Example:
///
/// ```bm_fnt
/// page id=0 file="bitmap_0.tga"
/// page id=1 file="bitmap_1.tga"
/// chars count=347
/// ```
///
/// Grammar:
///
/// ```grammar
/// Data      := Line*
/// Line      := Entry | Empty
/// Entry     := Tag Attribute* EOL   # e.g. page id=0 file="bm_0.png"
/// Empty     := EOL
/// Attribute := Key EQ Value         # e.g. count=32
/// Tag       := TW
/// Key       := TW
/// Value     := TW | TQ
/// TW        := WS string WN         # e.g. 1234
/// TQ        := QT string QT         # e.g. "my font.jpg", note string cannot contain QT | EOL.
/// WN        := WS | Null
/// EOL       := `CRLF` | `LF`        # End Of Line
/// WS        := `space` | `HT`
/// QT        := `"`
/// EQ        := `=`
/// Null      := ``
/// ```
///
/// Redundant WS characters are ignored.
///
/// Supports ASCII/ UTF8/ ISO 8859 encodings. Should also work with ASCII type formats that maintain
/// default values for control codes, `space` and `"` across all bytes.

pub struct TaggedAttributes<'a> {
    bytes: &'a [u8],
    index: usize,
    line: usize,
}

impl<'a> TaggedAttributes<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0, line: 1 }
    }

    #[inline(always)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline(always)]
    pub fn tag<'b>(&'b mut self) -> Result<Option<&'a [u8]>> {
        while let Some(byte) = self.skip() {
            if byte == CR {
                self.crlf(1)?;
                self.line += 1;
                continue;
            }
            if byte == LF {
                self.lf(1);
                self.line += 1;
                continue;
            }
            let head = self.index;
            self.index += 1;
            let tail = self.value_tail_wn()?;
            return Ok(Some(&self.bytes[head..tail]));
        }
        return Ok(None);
    }

    #[inline(always)]
    pub fn key_value<'b>(&'b mut self) -> Result<Option<(&'a [u8], &'a [u8])>> {
        if let Some(byte) = self.skip() {
            if byte == CR {
                self.crlf(0)?;
                return Ok(None);
            }
            if byte == LF {
                self.lf(0);
                return Ok(None);
            }
            let key_head = self.index;
            self.index += 1;
            let key_tail = self.key_tail()?;
            if let Some(byte) = self.skip() {
                let mut value_head = self.index;
                self.index += 1;
                let value_tail = match byte {
                    CR | LF => Err(Error::UnexpectedEndOfLine),
                    QT => {
                        value_head += 1;
                        self.value_tail_qt()
                    }
                    _ => self.value_tail_wn(),
                }?;
                Ok(Some((&self.bytes[key_head..key_tail], &self.bytes[value_head..value_tail])))
            } else {
                Err(Error::UnexpectedEndOfLine)
            }
        } else {
            Ok(None)
        }
    }

    #[inline(always)]
    fn key_tail(&mut self) -> Result<usize> {
        while let Some(byte) = self.byte() {
            if byte > SP {
                if byte == EQ {
                    let index = self.index;
                    self.index += 1;
                    return Ok(index);
                }
                self.index += 1;
                continue;
            }
            if byte == CR || byte == LF {
                return Err(Error::UnexpectedEndOfLine);
            }
            if byte == SP || byte == TB {
                let index = self.index;
                self.index += 1;
                while let Some(byte) = self.byte() {
                    self.index += 1;
                    if byte == EQ {
                        return Ok(index);
                    }
                    if byte != SP && byte != TB {
                        break;
                    }
                }
                return Err(Error::ExpectedEq);
            }
            self.index += 1;
        }
        Err(Error::UnexpectedEndOfFile)
    }

    #[inline(always)]
    fn value_tail_wn(&mut self) -> Result<usize> {
        while let Some(byte) = self.byte() {
            if byte == CR {
                let index = self.index;
                self.crlf(0)?;
                return Ok(index);
            }
            if byte == LF {
                let index = self.index;
                self.lf(0);
                return Ok(index);
            }
            if byte == SP || byte == TB {
                let index = self.index;
                self.index += 1;
                return Ok(index);
            }
            self.index += 1;
        }
        Ok(self.index)
    }

    #[inline(always)]
    fn value_tail_qt(&mut self) -> Result<usize> {
        while let Some(byte) = self.byte() {
            if byte == CR || byte == LF {
                return Err(Error::UnexpectedEndOfLine);
            }
            if byte == QT {
                let index = self.index;
                self.index += 1;
                return Ok(index);
            }
            self.index += 1;
        }
        Err(Error::UnexpectedEndOfFile)
    }

    #[inline(always)]
    fn skip(&mut self) -> Option<u8> {
        while let Some(byte) = self.byte() {
            if byte != SP && byte != TB {
                return Some(byte);
            }
            self.index += 1;
            continue;
        }
        None
    }

    #[inline(always)]
    fn crlf(&mut self, n: usize) -> Result<()> {
        self.index += 1;
        match self.byte() {
            Some(LF) => {
                self.lf(n);
                Ok(())
            }
            Some(_) | None => Err(Error::BadCRLF),
        }
    }

    #[inline(always)]
    fn lf(&mut self, n: usize) {
        self.index += n;
    }

    #[inline(always)]
    fn byte(&mut self) -> Option<u8> {
        if self.index < self.bytes.len() {
            Some(self.bytes[self.index])
        } else {
            None
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    BadCRLF,
    ExpectedEq,
    UnexpectedEndOfFile,
    UnexpectedEndOfLine,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadCRLF => f.write_str("bad new line"),
            Error::ExpectedEq => f.write_str("expected '='"),
            Error::UnexpectedEndOfFile => f.write_str("unexpected end of file"),
            Error::UnexpectedEndOfLine => f.write_str("unexpected end of line"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tag {
        ($name:ident, $bytes:expr, $line:expr) => {
            #[test]
            fn $name() -> Result<()> {
                let mut tkv = TaggedAttributes::from_bytes($bytes);
                assert_eq!(tkv.tag()?, Some(b"TAG".as_ref()));
                assert_eq!(tkv.line(), $line,);
                assert_eq!(tkv.key_value()?, None,);
                assert_eq!(tkv.line(), $line,);
                Ok(())
            }
        };
    }

    // Tag extraction tests
    tag!(tag, b"TAG", 1);
    tag!(sp_tag, b" TAG", 1);
    tag!(tb_tag, b"\tTAG", 1);
    tag!(lf_tag, b"\nTAG", 2);
    tag!(crlf_tag, b"\r\nTAG", 2);
    tag!(lf_crlf_tag, b"\n\r\nTAG", 3);
    tag!(crlf_lf_tag, b"\r\n\nTAG", 3);
    tag!(tag_sp, b"TAG ", 1);
    tag!(tag_tb, b"TAG\t", 1);
    tag!(tag_lf, b"TAG\n", 1);
    tag!(tag_crlf, b"TAG\r\n", 1);

    macro_rules! key_eq_value {
        ($name:ident, $bytes:expr) => {
            key_eq_value! { $name, $bytes, b"KEY", b"VALUE" }
        };

        ($name:ident, $bytes:expr, $key:expr, $value:expr) => {
            #[test]
            fn $name() -> Result<()> {
                let mut tkv = TaggedAttributes::from_bytes($bytes);
                assert_eq!(tkv.key_value()?, Some(($key.as_ref(), $value.as_ref())));
                assert_eq!(tkv.line(), 1);
                assert_eq!(tkv.key_value()?, None,);
                assert_eq!(tkv.line(), 1);
                Ok(())
            }
        };
    }

    // Key value pair (whitespace newline bound) extraction tests
    key_eq_value!(key_eq_value_wn, b"KEY=VALUE");
    key_eq_value!(sp_key_eq_value_wn, b" KEY=VALUE");
    key_eq_value!(tb_key_eq_value_wn, b"\tKEY=VALUE");
    key_eq_value!(key_sp_eq_value_wn, b"KEY =VALUE");
    key_eq_value!(key_tb_eq_value_wn, b"KEY\t=VALUE");
    key_eq_value!(key_eq_sp_value_wn, b" KEY= VALUE");
    key_eq_value!(key_eq_tb_value_wn, b"KEY=\tVALUE");
    key_eq_value!(key_eq_value_wn_sp, b"KEY=VALUE ");
    key_eq_value!(key_eq_value_wn_tb, b"KEY=VALUE\t");
    key_eq_value!(key_eq_value_wn_lf, b"KEY=VALUE\n");
    key_eq_value!(key_eq_value_wn_crlf, b"KEY=VALUE\r\n");

    // Key value pair (double quote bound) extraction tests
    key_eq_value!(key_eq_qt_value_qt, b"KEY=\"VALUE\"");
    key_eq_value!(sp_key_eq_qt_value_qt, b" KEY=\"VALUE\"");
    key_eq_value!(tb_key_eq_qt_value_qt, b"\tKEY=\"VALUE\"");
    key_eq_value!(key_sp_eq_qt_value_qt, b"KEY =\"VALUE\"");
    key_eq_value!(key_tb_eq_qt_value_qt, b"KEY\t=\"VALUE\"");
    key_eq_value!(key_eq_sp_qt_value_qt, b" KEY= \"VALUE\"");
    key_eq_value!(key_eq_tb_qt_value_qt, b"KEY=\t\"VALUE\"");
    key_eq_value!(key_eq_qt_value_qt_sp, b"KEY=\"VALUE\" ");
    key_eq_value!(key_eq_qt_value_qt_tb, b"KEY=\"VALUE\"\t");
    key_eq_value!(key_eq_qt_value_qt_lf, b"KEY=\"VALUE\"\n");
    key_eq_value!(key_eq_qt_value_qt_crlf, b"KEY=\"VALUE\"\r\n");

    // Key value pair (key quote/ eq variation) extraction tests
    key_eq_value!(key_qt_eq_qt_value_qt, b"KEY\"=\"VALUE\"", b"KEY\"", b"VALUE");
    key_eq_value!(qt_key_eq_qt_value_qt, b"\"KEY=\"VALUE\"", b"\"KEY", b"VALUE");
    key_eq_value!(qt_key_qt_eq_qt_value_qt, b"\"KEY\"=\"VALUE\"", b"\"KEY\"", b"VALUE");
    key_eq_value!(eq_key_eq_qt_value_qt, b"=KEY=\"VALUE\"", b"=KEY", b"VALUE");

    // Key value pair (value quote/ eq variation) extraction tests
    key_eq_value!(key_eq_value_qt, b"KEY=VALUE\"", b"KEY", b"VALUE\"");
    key_eq_value!(key_eq_eq, b"KEY==", b"KEY", b"=");
    key_eq_value!(key_eq_qt_eq_qt, b"KEY=\"=\"", b"KEY", b"=");
    key_eq_value!(key_eq_qt_qt, b"KEY=\"\"", b"KEY", b"");

    macro_rules! key_value_err {
        ($name:ident, $bytes:expr) => {
            key_eq_value! { $name, $bytes, b"KEY", b"VALUE" }
        };

        ($name:ident, $bytes:expr, $err:expr) => {
            #[test]
            fn $name() {
                let mut tkv = TaggedAttributes::from_bytes($bytes);
                match tkv.key_value() {
                    Err(err) => assert_eq!(err, $err),
                    Ok(_) => panic!("expect error: {}", $err),
                }
            }
        };
    }

    // Key value pair errors
    key_value_err!(key, b"KEY", Error::UnexpectedEndOfFile);
    key_value_err!(key_lf, b"KEY\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_crlf, b"KEY\r\n", Error::UnexpectedEndOfLine);
    key_value_err!(eq_value, b"=VALUE", Error::UnexpectedEndOfFile);
    key_value_err!(eq_value_lf, b"=VALUE\n", Error::UnexpectedEndOfLine);
    key_value_err!(eq_value_crlf, b"=VALUE\r\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq, b"KEY=", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq_qt, b"KEY=\"", Error::UnexpectedEndOfFile);
    key_value_err!(key_eq_qt_lf, b"KEY=\"\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq_qt_crlf, b"KEY=\"\r\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq_qt_value, b"KEY=\"VALUE", Error::UnexpectedEndOfFile);
    key_value_err!(key_eq_qt_value_lf_qt, b"KEY=\"VALUE\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq_qt_value_crlf_qt, b"KEY=\"VALUE\r\n", Error::UnexpectedEndOfLine);
    key_value_err!(key_eq_value_cr, b"KEY=VALUE\r", Error::BadCRLF);

    #[test]
    fn qt_key() -> Result<()> {
        let mut tkv = TaggedAttributes::from_bytes(b"\"KEY=VALUE");
        assert_eq!(tkv.key_value()?, Some(("\"KEY".as_ref(), "VALUE".as_ref())));
        Ok(())
    }

    #[test]
    fn key_qt() -> Result<()> {
        let mut tkv = TaggedAttributes::from_bytes(b"\"KEY=VALUE");
        assert_eq!(tkv.key_value()?, Some(("\"KEY".as_ref(), "VALUE".as_ref())));
        Ok(())
    }

    macro_rules! tagged_attribute {
        ($name:ident, $bytes:expr, $($v:expr),+) => {
            #[test]
            fn $name() -> Result<()> {
                let mut tkv = TaggedAttributes::from_bytes($bytes);
                assert_eq!(tkv.tag()?, Some(b"TAG".as_ref()));
                assert_eq!(tkv.line(), 1);
                let mut i = 0;
                $(
                    i += 1;
                    let key = format!("K{}", i);
                    assert_eq!(tkv.key_value()?, Some((key.as_bytes(), $v.as_ref())));
                    assert_eq!(tkv.line(), 1);
                )*
                assert_eq!(tkv.key_value()?, None,);
                assert_eq!(tkv.line(), 1);
                Ok(())
            }
        };
    }

    // Tag key value extraction tests, single line
    tagged_attribute!(tkv_wn, b"TAG K1=V1", "V1");
    tagged_attribute!(tkv_wn_wn, b"TAG K1=V1 K2=V2", "V1", "V2");
    tagged_attribute!(tkv_qt, b"TAG K1=\"V1\"", "V1");
    tagged_attribute!(tkv_qt_qt, b"TAG K1=\"V1\" K2=\"V2\"", "V1", "V2");
    tagged_attribute!(tkv_wn_qt, b"TAG K1=V1 K2=\"V2\"", "V1", "V2");
    tagged_attribute!(tkv_qt_wn, b"TAG K1=\"V1\" K2=V2", "V1", "V2");
    tagged_attribute!(tkv_qst_wn, b"TAG K1=\"V 1\" K2=V2", "V 1", "V2");

    macro_rules! tkvm {
        ($name:ident, $newline:expr, $line:expr) => {
            #[test]
            fn $name() -> Result<()> {
                let data = format!(
                    "{}\
                     TAG1 K1=V1{}\
                     TAG2 K2=V2 K3=\"V3\"",
                    $newline[0], $newline[1],
                );
                let mut tkv = TaggedAttributes::from_bytes(data.as_bytes());
                assert_eq!(tkv.tag()?, Some(b"TAG1".as_ref()));
                assert_eq!(tkv.line(), $line[0]);
                assert_eq!(tkv.key_value()?, Some((b"K1".as_ref(), b"V1".as_ref())));
                assert_eq!(tkv.line(), $line[0]);
                assert_eq!(tkv.key_value()?, None,);
                assert_eq!(tkv.line(), $line[0]);
                assert_eq!(tkv.tag()?, Some(b"TAG2".as_ref()));
                assert_eq!(tkv.line(), $line[1]);
                assert_eq!(tkv.key_value()?, Some((b"K2".as_ref(), b"V2".as_ref())));
                assert_eq!(tkv.line(), $line[1]);
                assert_eq!(tkv.key_value()?, Some((b"K3".as_ref(), b"V3".as_ref())));
                assert_eq!(tkv.line(), $line[1]);
                assert_eq!(tkv.key_value()?, None,);
                assert_eq!(tkv.line(), $line[1]);
                Ok(())
            }
        };
    }

    // Tag key value extraction tests, multiple lines
    tkvm!(newline_null_lf, ["", "\n"], [1, 2]);
    tkvm!(newline_lf_lf, ["\n", "\n"], [2, 3]);
    tkvm!(newline_crlf_lf, ["\r\n", "\n"], [2, 3]);
    tkvm!(newline_lflf_lf, ["\n\n", "\n"], [3, 4]);
    tkvm!(newline_crlfcrlf_lf, ["\r\n\r\n", "\n"], [3, 4]);
    tkvm!(newline_crlflf_lf, ["\r\n\n", "\n"], [3, 4]);
    tkvm!(newline_lfcrlf_lf, ["\n\r\n", "\n"], [3, 4]);
    tkvm!(newline_null_crlf, ["", "\r\n"], [1, 2]);
    tkvm!(newline_null_lflf, ["", "\n\n"], [1, 3]);
    tkvm!(newline_null_crlfcrlf, ["", "\r\n\r\n"], [1, 3]);
    tkvm!(newline_null_crlflf, ["", "\r\n\n"], [1, 3]);
    tkvm!(newline_null_lfcrlf, ["", "\n\r\n"], [1, 3]);

    // TODO fuzz
}
