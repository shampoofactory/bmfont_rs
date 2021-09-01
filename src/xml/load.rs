extern crate roxmltree as xml;

use crate::builder::attributes::{Attribute, Attributes};
use crate::builder::FontBuilder;
use crate::font::Font;
use crate::util;

use std::io;
use std::mem;

pub fn from_str(src: &str) -> crate::Result<Font> {
    FontBuilderXml::default().load_str(src)?.build()
}

pub fn from_bytes(bytes: &[u8]) -> crate::Result<Font> {
    from_str(util::utf8_str(None, bytes)?)
}

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
        let document = xml::Document::parse(src)?;
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
            tag_name => parse_error(format!("xml: invalid tag: {}", tag_name))?,
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
        if self.len() == 0 {
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

impl From<xml::Error> for crate::Error {
    fn from(err: xml::Error) -> Self {
        let line = Some(err.pos().row as usize);
        let err = format!("xml: {}", err);
        crate::Error::Parse { line, err }
    }
}

fn child_elements<F>(node: &xml::Node, mut op: F) -> crate::Result<()>
where
    F: FnMut(&xml::Node) -> crate::Result<()>,
{
    for child in node.children() {
        match child.node_type() {
            xml::NodeType::Root => internal_error("xml: nested root".to_owned())?,
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
        parse_error(format!("xml: invalid tag: '{}'", node_tag_name))
    }
}

fn check_null_attributes(node: &xml::Node) -> crate::Result<()> {
    if node.attributes().is_empty() {
        Ok(())
    } else {
        let tag_name = node.tag_name().name();
        parse_error(format!("xml: {}: unexpected attributes", tag_name))
    }
}

fn check_null_text(node: &xml::Node) -> crate::Result<()> {
    debug_assert_eq!(node.node_type(), xml::NodeType::Text);
    if let Some(text) = node.text() {
        if text.trim().is_empty() {
            Ok(())
        } else {
            parse_error(format!("xml: unexpected text: '{}'", text))
        }
    } else {
        internal_error("xml: text node: null text".to_owned())
    }
}

fn internal_error<T>(err: String) -> crate::Result<T> {
    Err(crate::Error::Internal { err })
}

fn parse_error<T>(err: String) -> crate::Result<T> {
    Err(crate::Error::Parse { line: None, err })
}
