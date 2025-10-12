extern crate roxmltree as xml;

use crate::builder::attributes::{Attribute, Attributes};
use crate::builder::FontBuilder;
use crate::font::Font;
use crate::LoadSettings;

use std::io;

/// Load XML format font.
///
/// Load a font from the specified XML format [str].
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
/// use std::fs;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let mut src = fs::read_to_string("font.xml")?;
///     let font = bmfont_rs::xml::from_str(&src)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_str(src: &str) -> crate::Result<Font> {
    from_str_ext(src, &Default::default())
}

/// Load XML format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_str_ext(src: &str, settings: &LoadSettings) -> crate::Result<Font> {
    FontBuilderXml::default().load_str(src, settings)?.build(settings)
}

/// Load XML format font.
///
/// Load a font from the specified XML format byte slice.
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
/// use std::fs;
///
/// fn main() -> bmfont_rs::Result<()> {
///     let mut buf = fs::read("font.xml")?;
///     let font = bmfont_rs::xml::from_bytes(&buf)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    from_bytes_ext(bytes, &Default::default())
}

/// Load XML format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_bytes_ext(bytes: &[u8], settings: &LoadSettings) -> crate::Result<Font> {
    from_str_ext(
        std::str::from_utf8(bytes).map_err(|e| crate::Error::Parse {
            line: None,
            entity: "font".to_owned(),
            err: e.to_string(),
        })?,
        settings,
    )
}

/// Read XML format font.
///
/// Read a font from the specified XML format reader.
/// This method buffers data internally, a buffered reader is not needed.
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
///     let mut f = File::open("font.xml")?;
///     let font = bmfont_rs::xml::from_reader(f)?;
///     println!("{:?}", font);
///     Ok(())
/// }
/// ```
pub fn from_reader<R: io::Read>(reader: R) -> crate::Result<Font> {
    from_reader_ext(reader, &Default::default())
}

/// Read XML format font with the specified import behavior settings.
///
/// This function specifies Font import behavior, allowing us to import certain partially
/// broken/ non-compliant BMFont files.
pub fn from_reader_ext<R: io::Read>(mut reader: R, settings: &LoadSettings) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes_ext(&vec, settings)
}

#[derive(Debug, Default)]
pub struct FontBuilderXml {
    builder: FontBuilder,
}

impl FontBuilderXml {
    pub fn load_str(mut self, src: &str, settings: &LoadSettings) -> crate::Result<FontBuilder> {
        let document = xml::Document::parse(src).map_err(|e| crate::Error::Parse {
            line: None,
            entity: "font".to_owned(),
            err: e.to_string(),
        })?;
        let root = document.root_element();
        check_tag_name(&root, "font")?;
        check_null_attributes(&root)?;
        child_elements(&root, |root| self.root_child(root, settings.ignore_invalid_tags))?;
        Ok(self.builder)
    }

    fn root_child(&mut self, node: &xml::Node, ignore_invalid_tags: bool) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        match node.tag_name().name() {
            "info" => self.info(node)?,
            "common" => self.common(node)?,
            "pages" => self.pages(node)?,
            "chars" => self.chars(node)?,
            "kernings" => self.kernings(node)?,
            tag_name => {
                if !ignore_invalid_tags {
                    return Err(crate::Error::InvalidTag { line: None, tag: tag_name.to_owned() });
                }
            }
        }
        Ok(())
    }

    fn info(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_info_attributes(None, &mut node.attributes())
    }

    fn common(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_common_attributes(None, &mut node.attributes())
    }

    fn pages(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        child_elements(node, |node| {
            check_tag_name(node, "page")?;
            self.builder.add_page_attributes(&mut node.attributes())
        })
    }

    fn chars(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_char_count_attributes(None, &mut node.attributes())?;
        child_elements(node, |node| {
            check_tag_name(node, "char")?;
            self.builder.add_char_attributes(&mut node.attributes())
        })
    }

    fn kernings(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_kerning_count_attributes(None, &mut node.attributes())?;
        child_elements(node, |node| {
            check_tag_name(node, "kerning")?;
            self.builder.add_kerning_attributes(&mut node.attributes())
        })
    }
}

impl<'a, 'input: 'a> Attributes<'a> for xml::Attributes<'a, 'input> {
    fn next_attribute(&mut self) -> crate::Result<Option<Attribute<'a>>> {
        Ok(self.next().map(|u| {
            let key = u.name().as_bytes();
            let value = u.value().as_bytes();
            Attribute::new(key, value, None)
        }))
    }
}

fn child_elements<F>(node: &xml::Node, mut op: F) -> crate::Result<()>
where
    F: FnMut(&xml::Node) -> crate::Result<()>,
{
    for child in node.children() {
        match child.node_type() {
            xml::NodeType::Root => {
                return Err(crate::Error::Internal { err: "xml: nested root".to_owned() })
            }
            xml::NodeType::Element => op(&child)?,
            xml::NodeType::PI | xml::NodeType::Comment => continue,
            xml::NodeType::Text => check_null_text(&child)?,
        }
    }
    Ok(())
}

fn check_tag_name(node: &xml::Node, tag_name: &str) -> crate::Result<()> {
    let node_tag_name = node.tag_name().name();
    if node_tag_name == tag_name {
        Ok(())
    } else {
        Err(crate::Error::InvalidTag { line: None, tag: tag_name.to_owned() })
    }
}

fn check_null_attributes(node: &xml::Node) -> crate::Result<()> {
    match node.attributes().len() {
        0 => Ok(()),
        _ => {
            let tag_name = node.tag_name().name();
            Err(crate::Error::Parse {
                line: None,
                entity: "xml".to_owned(),
                err: format!("{}: unexpected attributes", tag_name),
            })
        }
    }
}

fn check_null_text(node: &xml::Node) -> crate::Result<()> {
    debug_assert_eq!(node.node_type(), xml::NodeType::Text);
    if let Some(text) = node.text() {
        if text.trim().is_empty() {
            Ok(())
        } else {
            let tag_name = node.tag_name().name();
            Err(crate::Error::Parse {
                line: None,
                entity: "xml".to_owned(),
                err: format!("{}: unexpected text", tag_name),
            })
        }
    } else {
        Err(crate::Error::Internal { err: "xml: text node: null text".to_owned() })
    }
}
