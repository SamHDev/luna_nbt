use crate::{Tag, TagIdent};
use serde::Deserializer;
use serde::de::{Visitor, Error, Unexpected};
use crate::error::NBTError;

pub struct NBTDeserializer(Option<Tag>);

macro_rules! basic_type {
    ($tag: expr, $type: expr) => {

        match self.0 {
            Some(tag) = if let Tag::$tag(x) = tag {
                visitor.visit_$type(x)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_$tag,
                    when: "$type".to_string()
                })
            }
        }

    };
}

impl<'de> Deserializer<'de> for NBTDeserializer {
    type Error = NBTError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Some(tag) => match tag {
                Tag::Byte(v) => visitor.visit_i8(v),
                Tag::Short(v) => visitor.visit_i16(v),
                Tag::Int(v) => visitor.visit_i32(v),
                Tag::Long(v) => visitor.visit_i64(v),
                Tag::Float(v) => visitor.visit_f32(v),
                Tag::Double(v) => visitor.visit_f64(v),
                Tag::ByteArray(_array) => Err(Self::Error::custom("TODO! FIX THIS ISSUE. (ByteArray deserialize_any)")),
                Tag::String(v) => visitor.visit_string(v),
                Tag::List(array) => Err(Self::Error::custom("TODO! FIX THIS ISSUE. (ByteArray deserialize_any)")),
                Tag::Compound(_) => Err(Self::Error::custom("TODO! FIX THIS ISSUE. (ByteArray deserialize_any)")),
                Tag::IntArray(_) => Err(Self::Error::custom("TODO! FIX THIS ISSUE. (ByteArray deserialize_any)")),
                Tag::LongArray(_) => Err(Self::Error::custom("TODO! FIX THIS ISSUE. (ByteArray deserialize_any)")),
            }
            None => visitor.visit_none()
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(feature="serde_boolean")]
        if let Tag::Byte(x) = self.0 {
            visitor.visit_bool(x == 0x01i8)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Byte,
                when: "bool".to_string()
            })
        }

        #[cfg(not(feature="serde_boolean"))]
        return Err(NBTError::UnserializableType { type_name: "bool".to_string() })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Byte(x) = self.0 {
            visitor.visit_i8(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Byte,
                when: "i8".to_string()
            })
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Short(x) = self.0 {
            visitor.visit_i16(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Short,
                when: "i16".to_string()
            })
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Int(x) = self.0 {
            visitor.visit_i32(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Int,
                when: "i32".to_string()
            })
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Long(x) = self.0 {
            visitor.visit_i64(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Long,
                when: "i64".to_string()
            })
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(feature="serde_unsigned")]
        if let Tag::Byte(x) = self.0 {
            visitor.visit_u8(x as u8)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Byte,
                when: "i8 -> u8".to_string()
            })
        }

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType { type_name: "u8".to_string() })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(feature="serde_unsigned")]
        if let Tag::Short(x) = self.0 {
            visitor.visit_u16(x as u16)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Short,
                when: "i16 -> u16".to_string()
            })
        }

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType { type_name: "u16".to_string() })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(feature="serde_unsigned")]
        if let Tag::Int(x) = self.0 {
            visitor.visit_u32(x as u32)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Int,
                when: "i32 -> u32".to_string()
            })
        }

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType { type_name: "u32".to_string() })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(feature="serde_unsigned")]
        if let Tag::Long(x) = self.0 {
            visitor.visit_u64(x as u64)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Long,
                when: "i64 -> u64".to_string()
            })
        }

        #[cfg(not(feature="serde_unsigned"))]
            return Err(NBTError::UnserializableType { type_name: "u64".to_string() })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Float(x) = self.0 {
            visitor.visit_f32(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Float,
                when: "f32".to_string()
            })
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::Double(x) = self.0 {
            visitor.visit_f64(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_Double,
                when: "f64".to_string()
            })
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::String(x) = self.0 {
            if x.len() == 1 {
                visitor.visit_char(x.chars().nth(0).unwrap())
            } else {
                Err(NBTError::InvalidChar)
            }
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_String,
                when: "char".to_string()
            })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::String(x) = self.0 {
            visitor.visit_str(&x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_String,
                when: "str".to_string()
            })
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        if let Tag::String(x) = self.0 {
            visitor.visit_string(x)
        } else {
            Err(NBTError::InvalidType {
                found: self.0.ident(),
                expecting: TagIdent::TAG_String,
                when: "string".to_string()
            })
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        return Err(NBTError::UnserializableType {type_name: "bytes".to_string()})
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        return Err(NBTError::UnserializableType {type_name: "bytes".to_string()})
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }
}

