pub mod attributes;
pub mod load;
pub mod tags;

use crate::font::{Char, Common, Font, Info, Kerning, Page};
use crate::{Error, LoadSettings};

use attributes::Attributes;
use load::Load;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Count {
    pub count: u32,
}

#[derive(Debug, Default)]
pub struct FontProto {
    pub info: Option<Info>,
    pub common: Option<Common>,
    pub pages: Option<Vec<String>>,
    pub chars: Option<Vec<Char>>,
    pub kernings: Option<Vec<Kerning>>,
}

impl From<Font> for FontProto {
    fn from(value: Font) -> Self {
        let Font { info, common, pages, chars, kernings } = value;
        Self {
            info: Some(info),
            common: Some(common),
            pages: Some(pages),
            chars: Some(chars),
            kernings: Some(kernings),
        }
    }
}

impl FontProto {
    pub fn build_unchecked(mut self) -> crate::Result<Font> {
        let info = self.info.take().ok_or(Error::NoInfoBlock)?;
        let common = self.common.take().ok_or(Error::NoCommonBlock)?;
        let pages = self.pages.unwrap_or_default();
        let chars = self.chars.unwrap_or_default();
        let kernings = self.kernings.unwrap_or_default();
        Ok(Font::new(info, common, pages, chars, kernings))
    }

    pub fn build(self, settings: &LoadSettings) -> crate::Result<Font> {
        let font = self.build_unchecked()?;
        if !settings.ignore_counts {
            {
                let specified = font.common.pages;
                let realized = font.pages.len();
                if specified as usize != realized {
                    return Err(Error::InvalidPageCount { specified, realized });
                }
            }
        }
        if !settings.allow_string_control_characters {
            for page in &font.pages {
                check_string("page id", page)?;
            }
            check_string("info face", &font.info.face)?;
        }
        Ok(font)
    }

    pub fn set_info(&mut self, line: Option<usize>, info: Info) -> crate::Result<()> {
        if self.info.is_some() {
            Err(crate::Error::DuplicateInfoBlock { line })
        } else {
            self.info = Some(info);
            Ok(())
        }
    }

    pub fn set_common(&mut self, line: Option<usize>, common: Common) -> crate::Result<()> {
        if self.common.is_some() {
            Err(crate::Error::DuplicateCommonBlock { line })
        } else {
            self.common = Some(common);
            Ok(())
        }
    }

    pub fn set_pages(&mut self, _: Option<usize>, mut pages: Vec<String>) -> crate::Result<()> {
        if let Some(ref mut v) = self.pages {
            // Avoiding `Vec::extend_from_slice` with unnecessary String cloning.
            pages.drain(..).for_each(|u| v.push(u));
        } else {
            self.pages = Some(pages);
        }
        Ok(())
    }

    pub fn set_chars(&mut self, _: Option<usize>, chars: Vec<Char>) -> crate::Result<()> {
        if let Some(ref mut v) = self.chars {
            v.extend_from_slice(&chars);
        } else {
            self.chars = Some(chars);
        }
        Ok(())
    }

    pub fn set_kernings(&mut self, _: Option<usize>, kernings: Vec<Kerning>) -> crate::Result<()> {
        if let Some(ref mut v) = self.kernings {
            v.extend_from_slice(&kernings);
        } else {
            self.kernings = Some(kernings);
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FontBuilder {
    proto: FontProto,
    pages: Vec<String>,
    chars: Vec<Char>,
    char_count: Option<u32>,
    kernings: Vec<Kerning>,
    kerning_count: Option<u32>,
}

impl FontBuilder {
    pub fn build(self, settings: &LoadSettings) -> crate::Result<Font> {
        if !settings.ignore_counts {
            if let Some(specified) = self.char_count {
                let realized = self.chars.len();
                if specified as usize != realized {
                    return Err(Error::InvalidCharCount { specified, realized });
                }
            }
            if let Some(specified) = self.kerning_count {
                let realized = self.kernings.len();
                if specified as usize != realized {
                    return Err(Error::InvalidKerningCount { specified, realized });
                }
            }
        }
        let FontBuilder { mut proto, pages, chars, kernings, .. } = self;
        proto.set_pages(None, pages)?;
        proto.set_chars(None, chars)?;
        proto.set_kernings(None, kernings)?;
        proto.build(settings)
    }

    pub fn set_info_attributes<'b, A>(
        &mut self,
        line: Option<usize>,
        attributes: &mut A,
    ) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        self.proto.set_info(line, Info::load(attributes)?)
    }

    pub fn set_common_attributes<'b, A>(
        &mut self,
        line: Option<usize>,
        attributes: &mut A,
    ) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        self.proto.set_common(line, Common::load(attributes)?)
    }

    pub fn add_page_attributes<'b, A>(&mut self, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        self.add_page(Page::load(attributes)?)
    }

    pub fn add_page(&mut self, page: Page) -> crate::Result<()> {
        let Page { id, file } = page;
        if id as usize == self.pages.len() {
            self.pages.push(file);
            Ok(())
        } else {
            Err(crate::Error::BrokenPageList)
        }
    }

    pub fn add_char_attributes<'b, A>(&mut self, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        self.add_char(Char::load(attributes)?)
    }

    pub fn add_char(&mut self, char: Char) -> crate::Result<()> {
        self.chars.push(char);
        Ok(())
    }

    pub fn set_char_count_attributes<'b, A>(
        &mut self,
        line: Option<usize>,
        attributes: &mut A,
    ) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        Count::load(attributes).and_then(|Count { count }| self.set_char_count(line, count))
    }

    pub fn set_char_count(&mut self, line: Option<usize>, char_count: u32) -> crate::Result<()> {
        if self.char_count.is_some() {
            Err(Error::DuplicateCharCount { line })
        } else {
            self.char_count = Some(char_count);
            if self.chars.len() < char_count as usize {
                self.chars.reserve(char_count as usize - self.chars.len())
            }
            Ok(())
        }
    }

    pub fn set_kerning_count_attributes<'b, A>(
        &mut self,
        line: Option<usize>,
        attributes: &mut A,
    ) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        Count::load(attributes).and_then(|Count { count }| self.set_kerning_count(line, count))
    }

    pub fn set_kerning_count(
        &mut self,
        line: Option<usize>,
        kerning_count: u32,
    ) -> crate::Result<()> {
        if self.kerning_count.is_some() {
            Err(Error::DuplicateKerningCount { line })
        } else {
            self.kerning_count = Some(kerning_count);
            if self.kernings.len() < kerning_count as usize {
                self.kernings.reserve(kerning_count as usize - self.kernings.len())
            }
            Ok(())
        }
    }

    pub fn add_kerning_attributes<'b, A>(&mut self, attributes: &mut A) -> crate::Result<()>
    where
        A: Attributes<'b>,
    {
        self.add_kerning(Kerning::load(attributes)?)
    }

    pub fn add_kerning(&mut self, kerning: Kerning) -> crate::Result<()> {
        self.kernings.push(kerning);
        Ok(())
    }
}

fn check_string<'a>(path: &'a str, value: &'a str) -> crate::Result<&'a str> {
    for c in value.chars() {
        match c {
            '\x00'..='\x1F' | '\x7F' => {
                return Err(crate::Error::UnsafeValueString {
                    path: path.to_owned(),
                    value: value.to_owned(),
                })
            }
            _ => {}
        }
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_ok {
        ($name:ident, $str:expr) => {
            #[test]
            fn $name() -> crate::Result<()> {
                assert!(check_string("test", $str).is_ok());
                Ok(())
            }
        };
    }

    check_ok!(check_ok_null, "");
    check_ok!(check_ok_space, " ");
    check_ok!(check_ok_tilde, "~");
    check_ok!(check_ok_unicode_face, "☺");

    macro_rules! check_err {
        ($name:ident, $str:expr) => {
            #[test]
            fn $name() -> crate::Result<()> {
                assert!(check_string("test", $str).is_err());
                Ok(())
            }
        };
    }

    check_err!(check_err_nul, "\x00");
    check_err!(check_err_us, "\x1F");
    check_err!(check_err_del, "\x7F");
    check_err!(check_err_nl, "\n");
    check_err!(check_err_cr, "\r");
}
