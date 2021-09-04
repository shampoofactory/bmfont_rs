use crate::tagged_attributes::TaggedAttributes;

pub trait Attributes<'a> {
    /// Should not be called again after None
    fn next_attribute(&mut self) -> crate::Result<Option<Attribute<'a>>>;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
            Ok(u) => Ok(u.map(|(key, value)| Attribute::new(key, value, Some(self.line())))),
            Err(err) => Err(crate::Error::Parse {
                line: Some(self.line()),
                entity: "attribute".to_owned(),
                err: format!("attributes: {}", err),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tagged_attributes_next_attribute() -> crate::Result<()> {
        let mut attributes = TaggedAttributes::from_bytes(b"key=value");
        assert_eq!(attributes.next_attribute()?, Some(Attribute::new(b"key", b"value", Some(1))));
        Ok(())
    }

    #[test]
    fn tagged_attributes_next_attribute_err() -> crate::Result<()> {
        let mut attributes = TaggedAttributes::from_bytes(b"key=");
        match attributes.next_attribute() {
            Err(_) => {}
            Ok(_) => panic!("expected error"),
        }
        Ok(())
    }
}
