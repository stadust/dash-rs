use crate::serde::ser::error::Error;
use itoa::{Buffer, Integer};
use serde::{
    ser::{Error as _, Impossible, SerializeStruct},
    Serialize, Serializer,
};
use std::{fmt::Display, io::Write};

#[allow(missing_debug_implementations)]
pub struct IndexedSerializer<W> {
    delimiter: &'static [u8],
    writer: W,
    map_like: bool,

    /// Value indicating whether this serializer has already serialized something. This is used to
    /// check if we need to prepend the delimiter to the next field.
    ///
    /// Note that this field cannot simply be replaced in favor of a `writer.len() == 0` check. In
    /// case of list-like serialization the first field could be `None`, which is serialized to the
    /// empty string. In that case, a delimiter needs to be appended, but since the writer would
    /// still be empty, no delimiter would be added.
    is_start: bool,
}

impl<W> IndexedSerializer<W>
where
    W: Write,
{
    pub fn new(delimiter: &'static str, writer: W, map_like: bool) -> Self {
        IndexedSerializer {
            delimiter: delimiter.as_bytes(),
            writer,
            map_like,
            is_start: true,
        }
    }

    fn append_integer<I: Integer>(&mut self, int: I) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        let mut buffer = Buffer::new();
        self.writer.write(buffer.format(int).as_bytes()).map_err(Error::custom)?;

        Ok(())
    }

    fn append_display<D: Display>(&mut self, val: D) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        write!(&mut self.writer, "{}", val).map_err(Error::custom)?;

        Ok(())
    }

    fn append(&mut self, s: &str) -> Result<(), Error> {
        if self.is_start {
            self.is_start = false;
        } else {
            self.writer.write_all(self.delimiter)?;
        }

        self.writer.write_all(s.as_bytes())?;
        Ok(())
    }
}

impl<W: Write> Serializer for &mut IndexedSerializer<W> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Impossible<(), Error>;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.append(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.append_integer(v)
    }

    // Why we do not use dtoa or ryu here: Those libraries append an unneeded
    // '.0' suffix for floating point numbers that represent integers. Robtop's
    // formatting does not do this, and we'd like to match RobTop formatting
    // as closely as possible (if only so that roundtrip tests do not need
    // to deal with a myriad of exceptions).
    // Therefore, use the standard library, despite it being up to 4x slower
    // at printing floats. This is serialization, so performance is less of a
    // concern.
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.append_display(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.append_display(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        // We don't need allocations for appending a single char
        // A buffer of size 4 is always enough to encode a char
        let mut char_buffer: [u8; 4] = [0; 4];
        self.append(v.encode_utf8(&mut char_buffer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.append(v)
    }

    // Here we serialize bytes by base64 encoding them, so it's always valid in Geometry Dash's format
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use base64::{engine::general_purpose::URL_SAFE, write::EncoderWriter};
        let mut enc = EncoderWriter::new(&mut self.writer, &URL_SAFE);
        enc.write_all(v)?;
        enc.finish()?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(self.delimiter)?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_struct"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("serialize_unit_variant"))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::Unsupported("serialize_newtype_struct"))
    }

    fn serialize_newtype_variant<T>(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        Err(Error::Unsupported("serialize_newtype_variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::Unsupported("serialize_seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unsupported("serialize_tuple"))
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_struct"))
    }

    fn serialize_tuple_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unsupported("serialize_tuple_variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unsupported("serialize_map"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        // We don't store the struct name and the amount of fields doesn't matter
        Ok(self)
    }

    fn serialize_struct_variant(
        self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Unsupported("serialize_struct_variant"))
    }

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display + ?Sized,
    {
        Err(Error::Unsupported("collect_str"))
    }
}

impl<W: Write> SerializeStruct for &mut IndexedSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        if self.map_like {
            self.append(key)?;
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::Serializer;

    use super::IndexedSerializer;

    // Test that documents a robtop quirk: Floats without decimal part are represented as integer
    #[test]
    fn serialize_float_without_decimal_part_serializes_as_integer() {
        let mut buffer = Vec::new();
        let mut serializer = IndexedSerializer::new(":", &mut buffer, false);
        serializer.serialize_f64(11.0f64).unwrap();
        assert_eq!("11", std::str::from_utf8(buffer.as_slice()).unwrap());
    }
}
