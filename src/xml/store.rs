use crate::font::{Char, Common, Font, Info, Kerning};

use std::io;

/// Store XML format font.
///
/// Store a font into a [String] in XML format.
///
/// # Errors
///
/// * [Error](crate::Error) detailing the nature of any errors.
///
/// # Example
///
/// ```no_run
/// fn main() -> bmfont_rs::Result<()> {
///     let font = bmfont_rs::Font::default();
///     let string = bmfont_rs::xml::to_string(&font)?;
///     println!("{}", string);
///     Ok(())
/// }
/// ```
pub fn to_string(font: &Font) -> crate::Result<String> {
    let vec = to_vec(font)?;
    String::from_utf8(vec).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "font".to_owned(),
        err: format!("UTF8: {}", e),
    })
}

/// Store XML format font.
///
/// Store a font into a [Vec] in XML format.
///
/// # Errors
///
/// * [Error](crate::Error) detailing the nature of any errors.
///
/// # Example
///
/// ```no_run
/// fn main() -> bmfont_rs::Result<()> {
///     let font = bmfont_rs::Font::default();
///     let vec = bmfont_rs::xml::to_vec(&font)?;
///     println!("{:02X?}", font);
///     Ok(())
/// }
/// ```
pub fn to_vec(font: &Font) -> crate::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::default();
    to_writer(&mut vec, font)?;
    Ok(vec)
}

/// Write XML format font.
///
/// Write a font to the specified writer in XML format.
/// This method buffers data internally, a buffered writer is not needed.
///
/// # Errors
///
/// * [Error](crate::Error) detailing the nature of any errors.
///
/// # Example
///
/// ```no_run
/// use std::io;
/// use std::io::prelude::*;
/// use std::fs::File;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let font = bmfont_rs::Font::default();
///     let mut writer = File::create("font.xml")?;
///     bmfont_rs::xml::to_writer(&mut writer, &font)?;
///     Ok(())
/// }
/// ```
pub fn to_writer<W: io::Write>(mut writer: W, font: &Font) -> io::Result<()> {
    font.store(&mut writer)
}

trait StoreXml {
    fn store<W: io::Write>(&self, writer: W) -> io::Result<()>;
}

impl StoreXml for Font {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(writer, "<?xml version=\"1.0\"?>")?;
        writeln!(writer, "<font>")?;
        self.info.store(&mut writer)?;
        self.common.store(&mut writer)?;
        writeln!(writer, "  <pages>")?;
        self.pages
            .iter()
            .enumerate()
            .try_for_each(|(i, s)| write!(writer, "    <page id=\"{}\" file=\"{}\" />", i, s))?;
        writeln!(writer, "  </pages>")?;
        writeln!(writer, "  <chars count=\"{}\">", self.chars.len())?;
        self.chars.iter().try_for_each(|u| u.store(&mut writer))?;
        writeln!(writer, "  </chars>")?;
        writeln!(writer, "  <kernings count=\"{}\">", self.kernings.len())?;
        self.kernings.iter().try_for_each(|u| u.store(&mut writer))?;
        writeln!(writer, "  </kernings>")?;
        writeln!(writer, "</font>")?;
        Ok(())
    }
}

impl StoreXml for Char {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(
            writer,
            "    <char \
                     id=\"{}\" \
                     x=\"{}\" \
                     y=\"{}\" \
                     width=\"{}\" \
                     height=\"{}\" \
                     xoffset=\"{}\" \
                     yoffset=\"{}\" \
                     xadvance=\"{}\" \
                     page=\"{}\" \
                     chnl=\"{}\" \
                 />",
            self.id,
            self.x,
            self.y,
            self.width,
            self.height,
            self.xoffset,
            self.yoffset,
            self.xadvance,
            self.page,
            u8::from(self.chnl)
        )
    }
}

impl StoreXml for Common {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(
            writer,
            "  <common \
                   lineHeight=\"{}\" \
                   base=\"{}\" \
                   scaleW=\"{}\" \
                   scaleH=\"{}\" \
                   pages=\"{}\" \
                   packed=\"{}\" \
                   alphaChnl=\"{}\" \
                   redChnl=\"{}\" \
                   greenChnl=\"{}\" \
                   blueChnl=\"{}\" \
               />",
            self.line_height,
            self.base,
            self.scale_w,
            self.scale_h,
            self.pages,
            self.packed as u32,
            self.alpha_chnl as u8,
            self.red_chnl as u8,
            self.green_chnl as u8,
            self.blue_chnl as u8
        )
    }
}

impl StoreXml for Info {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(
            writer,
            "  <info \
                   face=\"{}\" \
                   size=\"{}\" \
                   bold=\"{}\" \
                   italic=\"{}\" \
                   charset=\"{}\" \
                   unicode=\"{}\" \
                   stretchH=\"{}\" \
                   smooth=\"{}\" \
                   aa=\"{}\" \
                   padding=\"{},{},{},{}\" \
                   spacing=\"{},{}\" \
                   outline=\"{}\" \
               />",
            self.face,
            self.size,
            self.bold as u32,
            self.italic as u32,
            self.charset,
            self.unicode as u32,
            self.stretch_h as u32,
            self.smooth as u32,
            self.aa as u32,
            self.padding.up,
            self.padding.right,
            self.padding.down,
            self.padding.left,
            self.spacing.horizontal,
            self.spacing.vertical,
            self.outline
        )
    }
}

impl StoreXml for Kerning {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(
            writer,
            "    <kerning first=\"{}\" second=\"{}\" amount=\"{}\" />",
            self.first, self.second, self.amount
        )
    }
}
