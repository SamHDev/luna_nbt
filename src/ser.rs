use serde::{Serializer, Serialize};
use crate::Tag;
use crate::error::NBTError;
use std::collections::HashMap;
use std::fmt::Display;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

pub struct NBTSerializer;

#[allow(unused_variables)]
impl Serializer for NBTSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;
    type SerializeSeq = NBTSeqSerializer;
    type SerializeTuple = NBTSeqSerializer;
    type SerializeTupleStruct = NBTSeqSerializer;
    type SerializeTupleVariant = NBTVariantSeqSerializer;
    type SerializeMap = NBTMapSerializer;
    type SerializeStruct = NBTStructSerializer;
    type SerializeStructVariant = NBTVariantStructSerializer;


    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature="serde_boolean")]
        return Ok(Some(Tag::Byte(v as i8)));

        #[cfg(not(feature="serde_boolean"))]
        return Err(NBTError::UnserializableType {type_name: "bool".to_string()})
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Byte(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Short(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Int(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Long(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature="serde_unsigned")]
            return Ok(Some(Tag::Byte(v as i8)));

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType {type_name: "u8".to_string()})
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature="serde_unsigned")]
            return Ok(Some(Tag::Short(v as i16)));

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType {type_name: "u16".to_string()})
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature="serde_unsigned")]
            return Ok(Some(Tag::Int(v as i32)));

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType {type_name: "u32".to_string()})
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature="serde_unsigned")]
            return Ok(Some(Tag::Long(v as i64)));

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType {type_name: "i64".to_string()})
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        return Ok(Some(Tag::Float(v)));
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        return Ok(Some(Tag::Double(v)));
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        return Ok(Some(Tag::String(v.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        return Ok(Some(Tag::String(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        return Err(NBTError::UnserializableType {type_name: "bytes".to_string()})
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::String(variant.to_string())))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        value.serialize(Self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        match Serialize::serialize(value, self)? {
            Some(x) => Ok(Some(external(&variant, x))),
            None => Ok(None)
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(NBTSeqSerializer::new())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(NBTSeqSerializer::new())
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(NBTSeqSerializer::new())
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(NBTVariantSeqSerializer::new(variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(NBTMapSerializer::new())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(NBTStructSerializer::new())
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(NBTVariantStructSerializer::new(variant))
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Display {
        Ok(Some(Tag::String(value.to_string())))
    }
}

pub struct NBTSeqSerializer {
    elements: Vec<Tag>,
}
impl NBTSeqSerializer {
    pub fn new() -> Self { NBTSeqSerializer { elements: Vec::new() } }
}

impl SerializeSeq for NBTSeqSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if let Some(value) = value.serialize(NBTSerializer)? {
            self.elements.push(value);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::List(self.elements)))
    }
}
impl SerializeTuple for NBTSeqSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        if let Some(value) = value.serialize(NBTSerializer)? {
            self.elements.push(value);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for NBTSeqSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        if let Some(value) = value.serialize(NBTSerializer)? {
            self.elements.push(value);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

pub struct NBTVariantSeqSerializer {
    variant: String,
    elements: Vec<Tag>
}
impl NBTVariantSeqSerializer {
    pub fn new(variant: &str) -> Self {
        Self {
            variant: variant.to_string(),
            elements: Vec::new()
        }
    }
}
impl SerializeTupleVariant for NBTVariantSeqSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        if let Some(value) = value.serialize(NBTSerializer)? {
            self.elements.push(value);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(external(&self.variant, Tag::List(self.elements))))
    }
}

pub fn external(name: &str, value: Tag) -> Tag {
    let mut map = HashMap::new();
    map.insert(name.to_string(), value);
    Tag::Compound(map)
}

pub struct NBTMapSerializer {
    map: HashMap<String, Tag>,
    key: Option<String>
}
impl NBTMapSerializer {
    pub fn new() -> Self { Self { map:HashMap::new(), key:None }}
}

impl SerializeMap for NBTMapSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if let Some(Tag::String(key)) = key.serialize(NBTSerializer)? {
            self.key = Some(key);
        };
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if let Some(key) = &self.key {
            if let Some(v) = value.serialize(NBTSerializer)? {
                self.map.insert(key.clone(), v);
            }
        };
        self.key = None;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Compound(self.map)))
    }
}


pub struct NBTStructSerializer {
    map: HashMap<String, Tag>
}
impl NBTStructSerializer {
    pub fn new() -> Self { Self { map:HashMap::new() }}
}
impl SerializeStruct for NBTStructSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if let Some(v) = value.serialize(NBTSerializer)? {
            self.map.insert(key.to_string(), v);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(Tag::Compound(self.map)))
    }
}

pub struct NBTVariantStructSerializer {
    map: HashMap<String, Tag>,
    variant: String
}
impl NBTVariantStructSerializer {
    pub fn new(variant: &str) -> Self { Self { map:HashMap::new(), variant: variant.to_string() }}
}
impl SerializeStructVariant for NBTVariantStructSerializer {
    type Ok = Option<Tag>;
    type Error = NBTError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        if let Some(v) = value.serialize(NBTSerializer)? {
            self.map.insert(key.to_string(), v);
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Some(external(&self.variant, Tag::Compound(self.map))))
    }
}