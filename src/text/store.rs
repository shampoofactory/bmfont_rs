use crate::font::{Char, Common, Font, Info, Kerning};

use std::io;

/// Store text format font.
///
/// Store a font into a [String] in text format.
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
///     let string = bmfont_rs::text::to_string(&font)?;
///     println!("{}", string);
///     Ok(())
/// }
/// ```
pub fn to_string(font: &Font) -> crate::Result<String> {
    let vec = to_vec(font)?;
    String::from_utf8(vec).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "font".to_owned(),
        err: e.to_string(),
    })
}

/// Store text format font.
///
/// Store a font into a [Vec] in text format.
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
///     let vec = bmfont_rs::text::to_vec(&font)?;
///     println!("{:02X?}", font);
///     Ok(())
/// }
/// ```
pub fn to_vec(font: &Font) -> crate::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::default();
    to_writer(&mut vec, font)?;
    Ok(vec)
}

/// Write text format font.
///
/// Write a font to the specified writer in binary format.
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
///     let mut writer = File::create("font.txt")?;
///     bmfont_rs::text::to_writer(&mut writer, &font)?;
///     Ok(())
/// }
/// ```
pub fn to_writer<W: io::Write>(mut writer: W, font: &Font) -> io::Result<()> {
    font.store(&mut writer)
}

trait StoreFnt {
    fn store<W: io::Write>(&self, writer: W) -> io::Result<()>;
}

impl StoreFnt for Font {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        self.info.store(&mut writer)?;
        self.common.store(&mut writer)?;
        self.pages
            .iter()
            .enumerate()
            .try_for_each(|(i, s)| write!(writer, "page id={} file=\"{}\"\r\n", i, s))?;
        write!(writer, "chars count={}\r\n", self.chars.len())?;
        self.chars.iter().try_for_each(|u| u.store(&mut writer))?;
        write!(writer, "kernings count={}\r\n", self.kernings.len())?;
        self.kernings.iter().try_for_each(|u| u.store(&mut writer))?;
        Ok(())
    }
}

impl StoreFnt for Char {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        write!(
            writer,
            "char id={:<4} \
                x={:<5} \
                y={:<5} \
                width={:<5} \
                height={:<5} \
                xoffset={:<5} \
                yoffset={:<5} \
                xadvance={:<5} \
                page={:<2} \
                chnl={:<2}\
                \r\n",
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

impl StoreFnt for Common {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        write!(
            writer,
            "common \
                lineHeight={} \
                base={} \
                scaleW={} \
                scaleH={} \
                pages={} \
                packed={} \
                alphaChnl={} \
                redChnl={} \
                greenChnl={} \
                blueChnl={}\
                \r\n",
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

impl StoreFnt for Info {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        write!(
            writer,
            "info \
                face=\"{}\" \
                size={} \
                bold={} \
                italic={} \
                charset=\"{}\" \
                unicode={} \
                stretchH={} \
                smooth={} \
                aa={} \
                padding={},{},{},{} \
                spacing={},{} \
                outline={}\
                \r\n",
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

impl StoreFnt for Kerning {
    fn store<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        write!(
            writer,
            "kerning first={:<3} second={:<3} amount={:<4}\r\n",
            self.first, self.second, self.amount
        )
    }
}
