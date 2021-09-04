use crate::tagged_attributes::TaggedAttributes;

pub trait Tags<'a> {
    /// Should not be called again after None
    fn next_tag(&mut self) -> crate::Result<Option<Tag<'a>>>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tag<'a> {
    pub tag: &'a [u8],
    pub line: Option<usize>,
}

impl<'a> Tag<'a> {
    #[inline(always)]
    pub fn new(tag: &'a [u8], line: Option<usize>) -> Self {
        Self { tag, line }
    }
}

impl<'a> Tags<'a> for TaggedAttributes<'a> {
    fn next_tag(&mut self) -> crate::Result<Option<Tag<'a>>> {
        match self.tag() {
            Ok(u) => Ok(u.map(|tag| Tag::new(tag, Some(self.line())))),
            Err(e) => Err(crate::Error::Parse {
                line: Some(self.line()),
                entity: "tag".to_owned(),
                err: e.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tagged_attributes_next_tag() -> crate::Result<()> {
        let mut tags = TaggedAttributes::from_bytes(b"tag");
        assert_eq!(tags.next_tag()?, Some(Tag::new(b"tag", Some(1))));
        Ok(())
    }

    #[test]
    fn tagged_attributes_next_tag_err() -> crate::Result<()> {
        let mut attributes = TaggedAttributes::from_bytes(b"\n\r");
        match attributes.next_tag() {
            Err(_) => {}
            Ok(_) => panic!("expected error"),
        }
        Ok(())
    }
}
