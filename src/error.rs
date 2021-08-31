use crate::tags::TagIdent;

#[derive(Debug)]
pub enum NBTError {
    IO { error: std::io::Error },
    InvalidList { found: TagIdent, expecting: TagIdent },
    InvalidTag { found: u8 },
    InvalidImplicit { found: TagIdent },
    StringError,
    UnexpectedEndTag,
    Custom(String),
    UnserializableType { type_name: String },
    InvalidType { found: TagIdent, expecting: TagIdent, when: String },
    InvalidChar,
    NoData { when: String }
}
pub type NBTResult<T> = Result<T, NBTError>;

pub(crate) fn digest_io<T>(r: Result<T, std::io::Error>) -> NBTResult<T> {
    match r {
        Ok(s) => Ok(s),
        Err(e) => Err(NBTError::IO { error: e})
    }
}


use std::fmt;

impl fmt::Display for NBTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            NBTError::IO { error } => f.write_str(&format!("An IO error occurred: {:?}", error)),
            NBTError::InvalidList { found, expecting } => f.write_str(&format!("Invalid List. Was expecting type {} but found {}", expecting, found)),
            NBTError::InvalidTag { found } => f.write_str(&format!("Invalid Tag Identifier with value {:02X}", found)),
            NBTError::InvalidImplicit { found } => f.write_str(&format!("NBT blob does not start with a compound tag. Found {} tag", found)),
            NBTError::StringError  => f.write_str(&format!("An error occurred while parsing a UTF-8/CESU8 string")),
            NBTError::UnexpectedEndTag => f.write_str(&format!("An Unexpected {} was read.", TagIdent::TAG_End)),
            NBTError::Custom(e) => f.write_str(e),
            NBTError::UnserializableType {type_name} => f.write_str(&format!("The type '{}' cannot be serialized into NBT", type_name)),
            NBTError::InvalidType { found, expecting, when } => f.write_str(&format!("Found tag {}, was expecting {} when deserializing {}", found, expecting, when)),
            NBTError::InvalidChar => f.write_str(&format!("Failed to deserialize char, length of {} was not 1", TagIdent::TAG_String)),
            NBTError::NoData {when} => f.write_str(&format!("A value was required when deserializing {}, but none was given.", when)),
        }
    }
}

impl std::error::Error for NBTError {}

#[cfg(feature= "serde")]
use serde::{ser::Error as SerializeError, de::Error as DeserializeError};

#[cfg(feature= "serde")]
impl SerializeError for NBTError {
    fn custom<T>(msg: T) -> Self where T: fmt::Display {
        Self::Custom(msg.to_string())
    }
}
#[cfg(feature= "serde")]
impl DeserializeError for NBTError {
    fn custom<T>(msg: T) -> Self where T: fmt::Display {
        Self::Custom(msg.to_string())
    }
}