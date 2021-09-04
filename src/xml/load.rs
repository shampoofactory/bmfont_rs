extern crate roxmltree as xml;

use crate::builder::attributes::{Attribute, Attributes};
use crate::builder::FontBuilder;
use crate::font::Font;

use std::io;
use std::mem;

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
    FontBuilderXml::default().load_str(src)?.build()
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
    from_str(std::str::from_utf8(bytes).map_err(|e| crate::Error::Parse {
        line: None,
        entity: "font".to_owned(),
        err: e.to_string(),
    })?)
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
pub fn from_reader<R: io::Read>(mut reader: R) -> crate::Result<Font> {
    let mut vec = Vec::default();
    reader.read_to_end(&mut vec)?;
    from_bytes(&vec)
}

#[derive(Debug, Default)]
pub struct FontBuilderXml {
    builder: FontBuilder,
}

impl FontBuilderXml {
    pub fn load_str(mut self, src: &str) -> crate::Result<FontBuilder> {
        let document = xml::Document::parse(src).map_err(|e| crate::Error::Parse {
            line: None,
            entity: "font".to_owned(),
            err: e.to_string(),
        })?;
        let root = document.root_element();
        check_tag_name(&root, "font")?;
        check_null_attributes(&root)?;
        child_elements(&root, |root| self.root_child(root))?;
        Ok(self.builder)
    }

    fn root_child(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        match node.tag_name().name() {
            "info" => self.info(node)?,
            "common" => self.common(node)?,
            "pages" => self.pages(node)?,
            "chars" => self.chars(node)?,
            "kernings" => self.kernings(node)?,
            tag_name => {
                return Err(crate::Error::InvalidTag { line: None, tag: tag_name.to_owned() });
            }
        }
        Ok(())
    }

    fn info(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_info(None, &mut node.attributes())
    }

    fn common(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.set_common(None, &mut node.attributes())
    }

    fn pages(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        child_elements(node, |node| {
            check_tag_name(node, "page")?;
            self.builder.page(None, &mut node.attributes())
        })
    }

    fn chars(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.chars(None, &mut node.attributes())?;
        child_elements(node, |node| {
            check_tag_name(node, "char")?;
            self.builder.char(&mut node.attributes())
        })
    }

    fn kernings(&mut self, node: &xml::Node) -> crate::Result<()> {
        debug_assert!(node.node_type() == xml::NodeType::Element);
        self.builder.kernings(None, &mut node.attributes())?;
        child_elements(node, |node| {
            check_tag_name(node, "kerning")?;
            self.builder.kerning(&mut node.attributes())
        })
    }
}

impl<'a, 'b: 'a> Attributes<'a> for &'b [xml::Attribute<'a>] {
    fn next_attribute(&mut self) -> crate::Result<Option<Attribute<'a>>> {
        if self.is_empty() {
            return Ok(None);
        }
        let (head, tail) = mem::take(self).split_at(1);
        *self = tail;
        let head = &head[0];
        let key = head.name().as_bytes();
        let value = head.value().as_bytes();
        Ok(Some(Attribute::new(key, value, None)))
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
    if node.attributes().is_empty() {
        Ok(())
    } else {
        let tag_name = node.tag_name().name();
        Err(crate::Error::Parse {
            line: None,
            entity: "xml".to_owned(),
            err: format!("{}: unexpected attributes", tag_name),
        })
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
