use std::collections::HashMap;
use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TagIdent {
    TAG_End = 0,
    TAG_Byte = 1,
    TAG_Short = 2,
    TAG_Int = 3,
    TAG_Long = 4,
    TAG_Float = 5,
    TAG_Double = 6,
    TAG_Byte_Array = 7,
    TAG_String = 8,
    TAG_List = 9,
    TAG_Compound = 10,
    TAG_Int_Array = 11,
    TAG_Long_Array = 12
}

impl TagIdent {
    pub fn parse(value: &u8) -> Option<TagIdent> {
        match value {
            0 => Some(TagIdent::TAG_End),
            1 => Some(TagIdent::TAG_Byte),
            2 => Some(TagIdent::TAG_Short),
            3 => Some(TagIdent::TAG_Int),
            4 => Some(TagIdent::TAG_Long),
            5 => Some(TagIdent::TAG_Float),
            6 => Some(TagIdent::TAG_Double),
            7 => Some(TagIdent::TAG_Byte_Array),
            8 => Some(TagIdent::TAG_String),
            9 => Some(TagIdent::TAG_List),
            10 => Some(TagIdent::TAG_Compound),
            11 => Some(TagIdent::TAG_Int_Array),
            12 => Some(TagIdent::TAG_Long_Array),
            _ => None
        }
    }
}

impl fmt::Display for TagIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TagIdent::TAG_End => f.write_str("TAG_End"),
            TagIdent::TAG_Byte => f.write_str("TAG_Byte"),
            TagIdent::TAG_Short => f.write_str("TAG_Short"),
            TagIdent::TAG_Int => f.write_str("TAG_Int"),
            TagIdent::TAG_Long  => f.write_str("TAG_Long"),
            TagIdent::TAG_Float => f.write_str("TAG_Float"),
            TagIdent::TAG_Double => f.write_str("TAG_Double"),
            TagIdent::TAG_Byte_Array => f.write_str("TAG_ByteArray"),
            TagIdent::TAG_String => f.write_str("TAG_String"),
            TagIdent::TAG_List => f.write_str("TAG_List"),
            TagIdent::TAG_Compound => f.write_str("TAG_Compound"),
            TagIdent::TAG_Int_Array => f.write_str("TAG_IntArray"),
            TagIdent::TAG_Long_Array => f.write_str("TAG_LongArray"),
        }
    }
}


#[derive(Debug)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl Tag {
    pub fn id(&self) -> u8 {
        match &self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(_) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12
        }
    }

    pub fn ident(&self) -> TagIdent {
        match &self {
            Tag::Byte(_) => TagIdent::TAG_Byte,
            Tag::Short(_) => TagIdent::TAG_Short,
            Tag::Int(_) => TagIdent::TAG_Int,
            Tag::Long(_) => TagIdent::TAG_Long,
            Tag::Float(_) => TagIdent::TAG_Float,
            Tag::Double(_) => TagIdent::TAG_Double,
            Tag::ByteArray(_) => TagIdent::TAG_Byte_Array,
            Tag::String(_) => TagIdent::TAG_String,
            Tag::List(_) => TagIdent::TAG_List,
            Tag::Compound(_) => TagIdent::TAG_Compound,
            Tag::IntArray(_) => TagIdent::TAG_Int_Array,
            Tag::LongArray(_) => TagIdent::TAG_Long_Array,
        }
    }
}