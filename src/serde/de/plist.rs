use std::fmt::{Debug, Display, Formatter, write};
use std::ops::Deref;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{de::Visitor, Deserializer};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error<'de> {
    Unexpected {
        got: Event<'de>,
        expected: String
    },
    Custom(String),
    Xml(quick_xml::Error)
}

impl<'de> std::error::Error for Error<'de> {}

impl<'de> Display for Error<'de> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unexpected { got, expected } => write!(f, "expected {}, got {:?}", expected, got),
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::Xml(inner) => write!(f, "{}", inner)
        }
    }
}

impl<'de> serde::de::Error for Error<'de> {
    fn custom<T>(msg: T) -> Self where T: Display {
        Error::Custom(msg.to_string())
    }
}

impl From<quick_xml::Error> for Error<'_> {
    fn from(value: quick_xml::Error) -> Self {
        Error::Xml(value)
    }
}

pub struct PlistDeserializer<'de> {
    reader: Reader<&'de [u8]>,
}

impl<'de> PlistDeserializer<'de> {
    pub fn new(input: &'de str) -> Result<Self, Error<'de>> {
        let mut deserializer = PlistDeserializer {
            reader: Reader::from_str(input)
        };

        match deserializer.reader.read_event()? {
            Event::Decl(_) => (),
            event => return Err(Error::Unexpected {got: event, expected: String::from("<?xml ...?>")})
        }

        deserializer.expect_tag_start("plist")?;
        deserializer.expect_tag_start("dict")?;

        Ok(deserializer)
    }

    fn expect_tag_start(&mut self, tag: &str) -> Result<(), Error<'de>> {
        match self.reader.read_event()? {
            Event::Start(bytes) if bytes.name().0 == tag.as_bytes() => Ok(()),
            event => Err(Error::Unexpected { got: event, expected: format!("<{} ...>", tag) })
        }
    }

    fn expect_text(&mut self) -> Result<Cow<'de, [u8]>, Error<'de>> {
        match self.reader.read_event()? {
            Event::Text(text) => Ok(text.into_inner()),
            event => Err(Error::Unexpected {got: event, expected: String::from("text content")})
        }
    }

    fn expect_tag_end(&mut self, tag: &str) -> Result<(), Error<'de>> {
        match self.reader.read_event()? {
            Event::End(bytes) if bytes.name().0 == tag.as_bytes() => Ok(()),
            event => Err(Error::Unexpected { got: event, expected: format!("</{}>", tag) })
        }
    }

    fn expect_simple_tag(&mut self, tag: &str) -> Result<Cow<'de, [u8]>, Error<'de>> {
        self.expect_tag_start(tag)?;
        let content = self.expect_text()?;
        self.expect_tag_end(tag)?;
        Ok(content)
    }

    fn expect_key(&mut self) -> Result<Cow<'de, [u8]>, Error<'de>> {
        self.expect_simple_tag("k")
    }
}

impl<'a, 'de> Deserializer<'de> for &'a mut PlistDeserializer<'de> {
    type Error = Error<'de>;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self, name: &'static str, fields: &'static [&'static str], visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self, name: &'static str, variants: &'static [&'static str], visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::serde::de::plist::PlistDeserializer;

    const INPUT: &str = r#"<?xml version="1.0"?><plist><dict><k>1</k><i>34</i></dict></plist>"#;

    #[test]
    fn test_deserialize() {
        let mut deserializer = PlistDeserializer::new(INPUT).unwrap();
    }
}