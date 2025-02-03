use crate::binary;
use crate::charset::Charset;
use crate::font::*;
use crate::text;
#[cfg(feature = "xml")]
use crate::xml;
use crate::LoadSettings;

use std::error::Error;
use std::result::Result;

fn small() -> Font {
    let info = Info {
        face: "Small Test".to_owned(),
        size: 32,
        bold: false,
        italic: false,
        charset: Charset::Null,
        unicode: true,
        stretch_h: 100,
        smooth: true,
        aa: 4,
        padding: Padding { up: 1, right: 2, down: 3, left: 4 },
        spacing: Spacing { horizontal: 5, vertical: 6 },
        outline: 7,
    };
    let common = Common {
        line_height: 32,
        base: 24,
        scale_w: 1024,
        scale_h: 2048,
        pages: 1,
        packed: false,
        alpha_chnl: Packing::Glyph,
        red_chnl: Packing::GlyphOutline,
        green_chnl: Packing::One,
        blue_chnl: Packing::Zero,
    };
    let pages = vec!["small_sheet_0.png".to_owned()];
    let chars = vec![
        Char {
            id: 10,
            x: 281,
            y: 9,
            width: 4,
            height: 7,
            xoffset: 2,
            yoffset: 24,
            xadvance: 8,
            page: 0,
            chnl: Chnl::ALL,
        },
        Char {
            id: 32,
            x: 0,
            y: 0,
            width: 7,
            height: 20,
            xoffset: 4,
            yoffset: 17,
            xadvance: 9,
            page: 0,
            chnl: Chnl::RED,
        },
    ];
    let kernings = vec![
        Kerning { first: 10, second: 32, amount: -2 },
        Kerning { first: 32, second: 10, amount: 1 },
    ];
    Font { info, common, pages, chars, kernings }
}

#[test]
fn binary_small_from_bytes() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.bin");
    assert_eq!(binary::from_bytes(src)?, small());
    Ok(())
}

#[test]
fn binary_small_from_reader() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.bin");
    assert_eq!(binary::from_reader(src.as_ref())?, small());
    Ok(())
}

#[test]
fn binary_small_to_vec() -> Result<(), Box<dyn Error>> {
    let vec = binary::to_vec(&small())?;
    assert_eq!(binary::from_bytes(&vec)?, small());
    Ok(())
}

#[test]
fn binary_small_to_writer() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    binary::to_writer(&mut vec, &small())?;
    assert_eq!(binary::from_bytes(&vec)?, small());
    Ok(())
}

#[test]
fn binary_multi_page() -> Result<(), Box<dyn Error>> {
    let multi_page = include_bytes!("../../data/ok/multi-page.bin");
    let font = binary::from_bytes(multi_page)?;
    assert_eq!(font.pages.len(), font.common.pages.into());
    Ok(())
}

#[test]
fn text_small_from_bytes() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.txt");
    assert_eq!(text::from_bytes(src)?, small());
    Ok(())
}

#[test]
fn text_small_from_str() -> Result<(), Box<dyn Error>> {
    let src = include_str!("../../data/ok/small.txt");
    assert_eq!(text::from_str(src)?, small());
    Ok(())
}

#[test]
fn text_small_from_reader() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.txt");
    assert_eq!(text::from_reader(src.as_ref())?, small());
    Ok(())
}

#[test]
fn text_small_to_vec() -> Result<(), Box<dyn Error>> {
    let vec = text::to_vec(&small())?;
    assert_eq!(text::from_bytes(&vec)?, small());
    Ok(())
}

#[test]
fn text_small_to_string() -> Result<(), Box<dyn Error>> {
    let string = text::to_string(&small())?;
    assert_eq!(text::from_bytes(string.as_bytes())?, small());
    Ok(())
}

#[test]
fn text_small_to_writer() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    text::to_writer(&mut vec, &small())?;
    assert_eq!(text::from_bytes(&vec)?, small());
    Ok(())
}

#[test]
fn text_small_invalid_face_string() -> Result<(), Box<dyn Error>> {
    let mut small = small();
    small.info.face = "\x00".to_owned();
    assert!(text::to_string(&small).is_err());
    Ok(())
}

#[test]
fn text_small_invalid_page_string() -> Result<(), Box<dyn Error>> {
    let mut small = small();
    small.pages[0] = "\x00".to_owned();
    assert!(text::to_string(&small).is_err());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_from_bytes() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.xml");
    assert_eq!(xml::from_bytes(src)?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_from_str() -> Result<(), Box<dyn Error>> {
    let src = include_str!("../../data/ok/small.xml");
    assert_eq!(xml::from_str(src)?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_from_reader() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/ok/small.xml");
    assert_eq!(xml::from_reader(src.as_ref())?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_to_vec() -> Result<(), Box<dyn Error>> {
    let vec = xml::to_vec(&small())?;
    assert_eq!(xml::from_bytes(&vec)?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_to_string() -> Result<(), Box<dyn Error>> {
    let string = xml::to_string(&small())?;
    assert_eq!(xml::from_bytes(string.as_bytes())?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_to_writer() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    xml::to_writer(&mut vec, &small())?;
    assert_eq!(xml::from_bytes(&vec)?, small());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_string_escape() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    let mut small = small();
    small.info.face = "<\"&'☺'&\">".to_owned();
    small.pages[0] = "<\"&'☺'&\">.png".to_owned();
    xml::to_writer(&mut vec, &small)?;
    assert_eq!(xml::from_bytes(&vec)?, small);
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_invalid_face_string() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    let mut small = small();
    small.info.face = "\x00".to_owned();
    assert!(xml::to_writer(&mut vec, &small).is_err());
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_small_invalid_page_string() -> Result<(), Box<dyn Error>> {
    let mut vec = Vec::default();
    let mut small = small();
    small.pages[0] = "\x00".to_owned();
    assert!(xml::to_writer(&mut vec, &small).is_err());
    Ok(())
}

#[test]
fn text_binary_med_cmp() -> Result<(), Box<dyn Error>> {
    let text_src = include_bytes!("../../data/ok/small.txt");
    let text_font = text::from_bytes(text_src)?;
    let bin_src = include_bytes!("../../data/ok/small.bin");
    let bin_font = binary::from_bytes(bin_src)?;
    assert_eq!(text_font, bin_font);
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_binary_med_cmp() -> Result<(), Box<dyn Error>> {
    let xml_src = include_bytes!("../../data/ok/small.xml");
    let xml_font = xml::from_bytes(xml_src)?;
    let bin_src = include_bytes!("../../data/ok/small.bin");
    let bin_font = binary::from_bytes(bin_src)?;
    assert_eq!(xml_font, bin_font);
    Ok(())
}

#[cfg(feature = "xml")]
#[test]
fn xml_text_med_cmp() -> Result<(), Box<dyn Error>> {
    let xml_src = include_bytes!("../../data/ok/small.xml");
    let xml_font = xml::from_bytes(xml_src)?;
    let text_src = include_bytes!("../../data/ok/small.txt");
    let text_font = text::from_bytes(text_src)?;
    assert_eq!(xml_font, text_font);
    Ok(())
}

#[test]
fn validate_small() -> crate::Result<()> {
    small().validate_references()
}

macro_rules! err {
    ($name:ident, $op:expr, $err:pat) => {
        #[test]
        fn $name() {
            match $op {
                Err($err) => {}
                Err(e) => panic!("unexpected error: {}", e),
                Ok(_) => panic!("error expected"),
            }
        }
    };
}

err!(
    text_duplicate_key,
    text::from_bytes(include_bytes!("../../data/bad/duplicate_key.txt").as_ref()),
    crate::Error::DuplicateKey { .. }
);

err!(
    text_invalid_char_count,
    text::from_bytes(include_bytes!("../../data/bad/invalid_char_count.txt").as_ref()),
    crate::Error::InvalidCharCount { .. }
);

err!(
    text_invalid_kerning_count,
    text::from_bytes(include_bytes!("../../data/bad/invalid_kerning_count.txt").as_ref()),
    crate::Error::InvalidKerningCount { .. }
);

err!(
    text_no_info_block,
    text::from_bytes(include_bytes!("../../data/bad/no_info.txt").as_ref()),
    crate::Error::NoInfoBlock
);

err!(
    text_no_common_block,
    text::from_bytes(include_bytes!("../../data/bad/no_common.txt").as_ref()),
    crate::Error::NoCommonBlock
);

err!(
    text_invalid_tag,
    text::from_bytes(include_bytes!("../../data/bad/invalid_tag.txt").as_ref()),
    crate::Error::InvalidTag { .. }
);

err!(
    text_invalid_value,
    text::from_bytes(include_bytes!("../../data/bad/bad_int.txt").as_ref()),
    crate::Error::Parse { .. }
);

err!(
    bin_underflow,
    binary::from_bytes(include_bytes!("../../data/bad/underflow.bin").as_ref()),
    crate::Error::Parse { .. }
);

err!(
    bin_overflow,
    binary::from_bytes(include_bytes!("../../data/bad/overflow.bin").as_ref()),
    crate::Error::Parse { .. }
);

err!(
    bin_unsupported,
    binary::from_bytes(include_bytes!("../../data/bad/unsupported.bin").as_ref()),
    crate::Error::UnsupportedBinaryVersion { version: 0xFF }
);

#[test]
fn load_settings_ignore_char_count() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/bad/invalid_char_count.txt");
    let settings = LoadSettings::default().ignore_counts();
    assert_eq!(text::from_bytes_ext(src, &settings)?, small());
    Ok(())
}

#[test]
fn load_settings_ignore_kerning_count() -> Result<(), Box<dyn Error>> {
    let src = include_bytes!("../../data/bad/invalid_kerning_count.txt");
    let settings = LoadSettings::default().ignore_counts();
    assert_eq!(text::from_bytes_ext(src, &settings)?, small());
    Ok(())
}
