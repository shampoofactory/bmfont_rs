use crate::tagged_attributes::TaggedAttributes;

pub trait Tags<'a> {
    /// Should not be called again after None
    fn next_tag(&mut self) -> crate::Result<Option<Tag<'a>>>;
}

#[derive(Clone, Copy, Debug, Default)]
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
