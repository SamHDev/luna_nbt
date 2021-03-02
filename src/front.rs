use std::io::{Write, Read, Cursor};
use crate::error::{NBTResult, NBTError};
use crate::tags::Tag;
use crate::encode::{write_tag, write_root};
use crate::blob::Blob;
use crate::decode::{read_tag, read_ident, read_root};
use serde::Serialize;
use crate::ser::NBTSerializer;
use crate::TagIdent;

// Write Trait
pub trait NBTWrite {
    fn write<W: Write>(&self, writer: &mut W) -> NBTResult<()>;

    fn encode(&self) -> NBTResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.write(&mut buffer)?;
        Ok(buffer)
    }
}

impl NBTWrite for Tag {
    fn write<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        write_tag(writer, &self)
    }
}
impl NBTWrite for Blob {
    fn write<W: Write>(&self, writer: &mut W) -> NBTResult<()> {
        write_root(writer, &self.root, &self.elements)
    }
}

// Read Trait
pub trait NBTRead: Sized {
    fn read<R: Read>(reader: &mut R) -> NBTResult<Self>;

    fn decode<B: AsRef<[u8]>>(data: B) -> NBTResult<Self> {
        Self::read(&mut Cursor::new(data.as_ref().to_vec()))
    }
}

impl NBTRead for Tag {
    fn read<R: Read>(reader: &mut R) -> NBTResult<Self> {
        let ident = read_ident(reader)?;
        read_tag(reader, &ident)
    }
}
impl NBTRead for Blob {
    fn read<R: Read>(reader: &mut R) -> NBTResult<Self> {
        let (name, elements) = read_root(reader)?;
        Ok(Self { root: name, elements })
    }
}

#[cfg(feature="with_serde")]
pub fn encode_tag<T: Serialize>(o: &T) -> NBTResult<Option<Tag>> {
    o.serialize(NBTSerializer)
}

#[cfg(feature="with_serde")]
pub fn encode_named<T: Serialize>(o: &T, name: &str) -> NBTResult<Blob> {
    match encode_tag(o)? {
        Some(tag) => if let Tag::Compound(map) = tag {
            Ok(Blob { elements: map, root: name.to_string() })
        } else {
            Err(NBTError::InvalidImplicit { found: tag.ident() })
        },
        // Not sure about this
        None => Err(NBTError::InvalidImplicit { found: TagIdent::TAG_End })
    }
}

#[cfg(feature="with_serde")]
pub fn encode<T: Serialize>(o: &T) -> NBTResult<Blob> {
    encode_named(o, "")
}
