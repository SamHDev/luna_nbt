use crate::{Tag, TagIdent};
use serde::Deserializer;
use serde::de::{Visitor, SeqAccess, DeserializeSeed, MapAccess, EnumAccess, VariantAccess};
use crate::error::NBTError;
use std::collections::HashMap;

pub struct NBTDeserializer(Option<Tag>);

impl NBTDeserializer {
    pub fn new(s: Option<Tag>) -> Self { Self(s) }
    pub fn some(t: Tag) -> Self { Self(Some(t)) }
}

macro_rules! basic_type {
    ($value: ident, $visitor: ident, $tag: ident, $ident: ident, $func: ident, $name: expr) => {

        return match $value.0 {
            Some(tag) => if let Tag::$tag(x) = tag {
                $visitor.$func(x)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::$ident,
                    when: $name.to_string()
                })
            },
            None => Err(NBTError::NoData { when: $name.to_string() })
        };

    };
}

#[cfg(feature="serde_unsigned")]
macro_rules! unsigned_type {
    ($value: tt, $visitor: ident, $tag: ident, $ident: ident, $func: ident, $cast: ty, $name: literal) => {
        return match $value.0 {
            Some(tag) => if let Tag::$tag(x) = tag {
                $visitor.$func(x as $cast)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::$ident,
                    when: $name.to_string()
                })
            },
            None => Err(NBTError::NoData { when: $name.to_string() })
        };
    };
}

#[cfg(not(feature="serde_unsigned"))]
macro_rules! unsigned_type {
    ($value: ident, $visitor: ident, $tag: ident, $ident: ident, $func: ident, $cast: ty, $name: expr) => {
        return Err(NBTError::UnserializableType { type_name: $name.to_string() });
    }
}


#[allow(unused_variables)]
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
                Tag::ByteArray(list) => visitor.visit_seq(NBTSeqAccess::new(list.into_iter().map(|x| Tag::Byte(x)).collect())),
                Tag::String(v) => visitor.visit_string(v),
                Tag::List(array) => visitor.visit_seq(NBTSeqAccess::new(array)),
                Tag::Compound(compound) => visitor.visit_map(NBTMapAccess::new(compound)),
                Tag::IntArray(list) => visitor.visit_seq(NBTSeqAccess::new(list.into_iter().map(|x| Tag::Int(x)).collect())),
                Tag::LongArray(list) => visitor.visit_seq(NBTSeqAccess::new(list.into_iter().map(|x| Tag::Long(x)).collect())),
            }
            None => visitor.visit_none()
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        #[cfg(not(feature="serde_boolean"))]
            return Err(NBTError::UnserializableType { type_name: "bool".to_string() });

        #[cfg(feature="serde_boolean")]
        match self.0 {
            Some(tag) => if let Tag::Byte(x) = tag {
                visitor.visit_bool(x == 0x01i8)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_Byte,
                    when: "bool".to_string()
                })
            }
            None => Err(NBTError::NoData { when: "char".to_string() })
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        basic_type!(self, visitor, Byte, TAG_Byte, visit_i8, "i8");
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        basic_type!(self, visitor, Short, TAG_Short, visit_i16, "i16");
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        basic_type!(self, visitor, Int, TAG_Int, visit_i32, "i32");
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        basic_type!(self, visitor, Long, TAG_Long, visit_i64, "i64");
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        unsigned_type!(self, visitor, Byte, TAG_Byte, visit_u8, u8, "u8")
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        unsigned_type!(self, visitor, Short, TAG_Short, visit_u16, u16, "u16")
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        unsigned_type!(self, visitor, Int, TAG_Int, visit_u32, u32, "u32")
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unsigned_type!(self, visitor, Long, TAG_Long, visit_u64, u64, "u64")
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        basic_type!(self, visitor, Float, TAG_Float, visit_f32, "f32");
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        basic_type!(self, visitor, Double, TAG_Double, visit_f64, "f64");
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Some(tag) => if let Tag::String(x) = tag {
                if x.len() == 1 {
                    visitor.visit_char(x.chars().nth(0).unwrap())
                } else {
                    Err(NBTError::InvalidChar)
                }
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_String,
                    when: "char".to_string()
                })
            },
            None => Err(NBTError::NoData { when: "char".to_string() })
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Some(tag) => if let Tag::String(x) = tag {
                visitor.visit_str(&x)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_String,
                    when: "char".to_string()
                })
            },
            None => Err(NBTError::NoData { when: "char".to_string() })
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            Some(tag) => if let Tag::String(x) = tag {
                visitor.visit_string(x)
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_String,
                    when: "char".to_string()
                })
            },
            None => Err(NBTError::NoData { when: "char".to_string() })
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
        match self.0 {
            None => visitor.visit_none(),
            Some(t) => visitor.visit_some(Self::some(t))
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => visitor.visit_unit(),
            Some(t) => Err(NBTError::InvalidType {
                found: t.ident(),
                expecting: TagIdent::TAG_End,
                when: "unit".to_string()
            })
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "newtype_struct".to_string() }),
            Some(t) => visitor.visit_newtype_struct(NBTDeserializer::some(t))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "seq".to_string() }),
            Some(data) => if let Tag::List(list) = data {
                visitor.visit_seq(NBTSeqAccess::new(list))
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_List,
                    when: "seq".to_string()
                })
            }
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "tuple".to_string() }),
            Some(data) => if let Tag::List(list) = data {
                visitor.visit_seq(NBTSeqAccess::new(list))
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_List,
                    when: "tuple".to_string()
                })
            }
        }
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "tuple".to_string() }),
            Some(data) => if let Tag::List(list) = data {
                visitor.visit_seq(NBTSeqAccess::new(list))
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_List,
                    when: "tuple".to_string()
                })
            }
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "tag".to_string() }),
            Some(data) => if let Tag::Compound(comp) = data {
                visitor.visit_map(NBTMapAccess::new(comp))
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_Compound,
                    when: "map".to_string()
                })
            }
        }
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "struct".to_string() }),
            Some(data) => if let Tag::Compound(comp) = data {
                visitor.visit_map(NBTMapAccess::new(comp))
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_Compound,
                    when: "struct".to_string()
                })
            }
        }
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        visitor.visit_enum(NBTEnumAccess::new(self.0))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        match self.0 {
            None => Err(NBTError::NoData { when: "identifier".to_string() }),
            Some(data) => if let Tag::String(name) = data {
                visitor.visit_str(&name)
            } else {
                Err(NBTError::InvalidType {
                    found: data.ident(),
                    expecting: TagIdent::TAG_String,
                    when: "identifier".to_string()
                })
            }
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        unimplemented!()
    }
}

pub struct NBTSeqAccess {
    data: Vec<Tag>
}

impl NBTSeqAccess {
    pub fn new(s: Vec<Tag>) -> Self {
        Self {
            data: s
        }
    }
}

impl<'de> SeqAccess<'de> for NBTSeqAccess {
    type Error = NBTError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error> where
        T: DeserializeSeed<'de> {
        if self.data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(NBTDeserializer::some(self.data.remove(0)))?))
        }
    }
}

pub struct NBTMapAccess {
    data: Vec<(String, Tag)>,
    value: Option<Tag>
}

impl NBTMapAccess {
    pub fn new(s: HashMap<String, Tag>) -> Self {
        Self {
            data: s.into_iter().collect(),
            value: None
        }
    }
}


impl<'de> MapAccess<'de> for NBTMapAccess {
    type Error = NBTError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<<K as DeserializeSeed<'de>>::Value>, Self::Error> where
        K: DeserializeSeed<'de> {
        if self.data.is_empty() {
            Ok(None)
        } else {
            let (key, value) = self.data.remove(0);
            self.value = Some(value);
            Ok(Some(seed.deserialize(NBTDeserializer::some(Tag::String(key)))?))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<<V as DeserializeSeed<'de>>::Value, Self::Error> where
        V: DeserializeSeed<'de> {
        seed.deserialize(NBTDeserializer::new(std::mem::replace(&mut self.value, None)))
    }
}

pub struct NBTEnumAccess {
    content: Option<Tag>
}

impl NBTEnumAccess {
    pub fn new(s: Option<Tag>) -> Self {
        Self { content: s }
    }
}

impl<'de> EnumAccess<'de> for NBTEnumAccess {
    type Error = NBTError;
    type Variant = NBTEnumAccess;

    fn variant_seed<V>(self, seed: V) -> Result<(<V as DeserializeSeed<'de>>::Value, Self::Variant), Self::Error> where
        V: DeserializeSeed<'de> {

        match self.content {
            Some(tag) => if let Tag::Compound(map) = tag {
                if let Some((key, value)) = map.into_iter().nth(0) {
                    let seed = seed.deserialize(NBTDeserializer::some(Tag::String(key)))?;
                    let access = NBTEnumAccess::new(Some(value));
                    Ok((seed, access))
                } else {
                    Err(NBTError::InvalidType {
                        found: TagIdent::TAG_End,
                        expecting: TagIdent::TAG_Compound,
                        when: "enum map".to_string()
                    })
                }
            } else {
                Err(NBTError::InvalidType {
                    found: tag.ident(),
                    expecting: TagIdent::TAG_Compound,
                    when: "enum map".to_string()
                })
            },
            None => Err(NBTError::NoData { when: "enum map".to_string() })
        }
    }
}

impl<'de> VariantAccess<'de> for NBTEnumAccess {
    type Error = NBTError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error> where
        T: DeserializeSeed<'de> {
        seed.deserialize(NBTDeserializer::new(self.content))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        NBTDeserializer::new(self.content).deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where
        V: Visitor<'de> {
        NBTDeserializer::new(self.content).deserialize_struct("", fields, visitor)
    }
}