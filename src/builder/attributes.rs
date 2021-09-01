use crate::tagged_attributes::TaggedAttributes;

pub trait Attributes<'a> {
    /// Should not be called again after None
    fn next_attribute(&mut self) -> crate::Result<Option<Attribute<'a>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Attribute<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
    pub line: Option<usize>,
}

impl<'a> Attribute<'a> {
    #[inline(always)]
    pub fn new(key: &'a [u8], value: &'a [u8], line: Option<usize>) -> Self {
        Self { key, value, line }
    }
}

impl<'a> Attributes<'a> for TaggedAttributes<'a> {
    fn next_attribute(&mut self) -> crate::Result<Option<Attribute<'a>>> {
        match self.key_value() {
            Ok(u) => Ok(match u {
                Some((key, value)) => Some(Attribute::new(key, value, Some(self.line()))),
                None => None,
            }),
            Err(err) => Err(crate::Error::Parse {
                line: Some(self.line()),
                err: format!("attributes: {}", err),
            }),
        }
    }
}
