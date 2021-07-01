use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum SerErrorKind {
    Other(anyhow::Error),
}

#[derive(Debug)]
pub struct SerError {
    kind: SerErrorKind,
}

impl SerError {
    fn custom_with_ser<T: std::fmt::Display, W: std::io::Write>(
        msg: T,
        ser: &BencodeSerializer<W>,
    ) -> Self {
        serde::ser::Error::custom(msg)
    }
    fn from_err_with_ser<E: std::error::Error + Send + Sync + 'static, W: std::io::Write>(
        err: E,
        ser: &BencodeSerializer<W>,
    ) -> Self {
        Self {
            kind: SerErrorKind::Other(err.into()),
        }
    }
}

impl serde::ser::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            kind: SerErrorKind::Other(anyhow::anyhow!("{}", msg)),
        }
    }
}

impl std::error::Error for SerError {}

impl std::fmt::Display for SerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct BencodeSerializer<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> BencodeSerializer<W> {
    fn write_fmt(&mut self, fmt: core::fmt::Arguments<'_>) -> Result<(), SerError> {
        self.writer
            .write_fmt(fmt)
            .map_err(|e| SerError::from_err_with_ser(e, &self))
    }
    fn write_byte(&mut self, byte: u8) -> Result<(), SerError> {
        self.writer
            .write_all(&[byte])
            .map_err(|e| SerError::from_err_with_ser(e, &self))
    }
    fn write_number<N: std::fmt::Display>(&mut self, number: N) -> Result<(), SerError> {
        self.write_fmt(format_args!("i{}e", number))
    }
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), SerError> {
        self.write_fmt(format_args!("{}:", bytes.len()))?;
        self.writer
            .write_all(bytes)
            .map_err(|e| SerError::from_err_with_ser(e, &self))
    }
}

struct SerializeSeq<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeSeq for SerializeSeq<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_byte(b'e')
    }
}

struct SerializeTuple<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeTuple for SerializeTuple<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_byte(b'e')
    }
}

struct SerializeTupleStruct<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeTupleStruct for SerializeTupleStruct<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct SerializeTupleVariant<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeTupleVariant for SerializeTupleVariant<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

struct SerializeMap<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeMap for SerializeMap<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_byte(b'e')
    }
}

struct SerializeStruct<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeStruct for SerializeStruct<'ser, W> {
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.ser.write_bytes(key.as_bytes())?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.write_byte(b'e')
    }
}

struct SerializeStructVariant<'ser, W: std::io::Write> {
    ser: &'ser mut BencodeSerializer<W>,
}
impl<'ser, W: std::io::Write> serde::ser::SerializeStructVariant
    for SerializeStructVariant<'ser, W>
{
    type Ok = ();

    type Error = SerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'ser, W: std::io::Write> Serializer for &'ser mut BencodeSerializer<W> {
    type Ok = ();

    type Error = SerError;

    type SerializeSeq = SerializeSeq<'ser, W>;

    type SerializeTuple = SerializeTuple<'ser, W>;

    type SerializeTupleStruct = SerializeTupleStruct<'ser, W>;

    type SerializeTupleVariant = SerializeTupleVariant<'ser, W>;

    type SerializeMap = SerializeMap<'ser, W>;

    type SerializeStruct = SerializeStruct<'ser, W>;

    type SerializeStructVariant = SerializeStructVariant<'ser, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support booleans",
            &self,
        ))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support f32",
            &self,
        ))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support f32",
            &self,
        ))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support chars",
            &self,
        ))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.write_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support None",
            &self,
        ))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom_with_ser(
            "bencode doesn't support Rust unit ()",
            &self,
        ))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.write_byte(b'l')?;
        Ok(SerializeSeq { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.write_byte(b'd')?;
        Ok(SerializeMap { ser: self })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.write_byte(b'd')?;
        Ok(SerializeStruct { ser: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub fn bencode_serialize_to_writer<T: Serialize, W: std::io::Write>(
    value: T,
    writer: &mut W,
) -> Result<(), SerError> {
    let mut serializer = BencodeSerializer { writer };
    value.serialize(&mut serializer)?;
    Ok(())
}
