use std::io::Read;
use std::collections::HashMap;
use crate::{Tag, TagIdent};
use byteorder::{ReadBytesExt, BE};
use crate::error::{digest_io, NBTResult, NBTError};
use cesu8::Cesu8DecodingError;
use std::borrow::Cow;

pub(crate) fn read_ident<R: Read>(reader: &mut R) -> NBTResult<TagIdent> {
    let byte = digest_io(reader.read_u8())?;
    match TagIdent::parse(&byte) {
        Some(x) => Ok(x),
        None => Err(NBTError::InvalidTag { found: byte })
    }
}

pub fn read_root<R: Read>(reader: &mut R) -> NBTResult<(String, HashMap<String, Tag>)> {
    let implicit_ident = read_ident(reader)?;
    if implicit_ident != TagIdent::TAG_Compound {
        return Err(NBTError::InvalidImplicit { found: implicit_ident });
    };

    let name = read_string(reader)?;

    let compound = read_compound(reader)?;

    Ok((name, compound))

}

pub(crate) fn read_size<R: Read, S: Into<usize>>(reader: &mut R, size: S) -> NBTResult<Vec<u8>> {
    let size = size.into();
    let mut buffer = Vec::with_capacity(size.clone());
    for _ in 0..size {
        buffer.push(digest_io(reader.read_u8())?);
    }
    Ok(buffer)
}

pub(crate) fn read_string<R: Read>(reader: &mut R) -> NBTResult<String> {
    let length = digest_io(reader.read_u16::<BE>())?;

    let buffer = read_size(reader, length)?;

    decode_wonky_string(buffer)
}

pub(crate) fn read_compound<R: Read>(reader: &mut R) -> NBTResult<HashMap<String, Tag>> {
    let mut compound = HashMap::new();
    loop {
        let ident = read_ident(reader)?;
        if ident == TagIdent::TAG_End { break; }

        let name = read_string(reader)?;
        let payload = read_tag(reader, &ident)?;

        compound.insert(name, payload);
    }
    Ok(compound)
}

pub fn read_tag<R: Read>(reader: &mut R, ident: &TagIdent) -> NBTResult<Tag> {
    match ident {
        // If we get a end tag, we error.
        TagIdent::TAG_End => Err(NBTError::UnexpectedEndTag {}),

        // read byte (i8)
        TagIdent::TAG_Byte => Ok(Tag::Byte(digest_io(reader.read_i8())?)),

        // read short (i16)
        TagIdent::TAG_Short => Ok(Tag::Short(digest_io(reader.read_i16::<BE>())?)),

        // read int (i32)
        TagIdent::TAG_Int => Ok(Tag::Int(digest_io(reader.read_i32::<BE>())?)),

        // read long (i64)
        TagIdent::TAG_Long => Ok(Tag::Long(digest_io(reader.read_i64::<BE>())?)),

        // read float (f32)
        TagIdent::TAG_Float => Ok(Tag::Float(digest_io(reader.read_f32::<BE>())?)),

        // read double (f64)
        TagIdent::TAG_Double => Ok(Tag::Double(digest_io(reader.read_f64::<BE>())?)),

        // read byte array
        TagIdent::TAG_Byte_Array => {
            // get length int
            let length = digest_io(reader.read_u32::<BE>())?;

            // empty build array
            let mut array = Vec::new();

            // read items
            for _ in 0..length {
                array.push(digest_io(reader.read_i8())?)
            }
            Ok(Tag::ByteArray(array))
        }

        // read string
        TagIdent::TAG_String => Ok(Tag::String(read_string(reader)?)),

        // read list
        TagIdent::TAG_List => {
            // read list type
            let ident = read_ident(reader)?;

            // read length
            let length = digest_io(reader.read_u32::<BE>())?;

            // create empty buffer
            let mut list = Vec::new();

            // read items
            for _ in 0..length {
                list.push(read_tag(reader, &ident)?);
            }

            Ok(Tag::List(list))
        }

        // read compound
        TagIdent::TAG_Compound => Ok(Tag::Compound(read_compound(reader)?)),

        TagIdent::TAG_Int_Array => {
            // get length int
            let length = digest_io(reader.read_u32::<BE>())?;

            // empty build array
            let mut array = Vec::new();

            // read items
            for _ in 0..length {
                array.push(digest_io(reader.read_i32::<BE>())?)
            }
            Ok(Tag::IntArray(array))
        }
        TagIdent::TAG_Long_Array => {
            // get length int
            let length = digest_io(reader.read_u32::<BE>())?;

            // empty build array
            let mut array = Vec::new();

            // read items
            for _ in 0..length {
                array.push(digest_io(reader.read_i64::<BE>())?)
            }
            Ok(Tag::LongArray(array))
        }
    }
}

pub (crate) fn decode_wonky_string(b: &[u8]) -> NBTResult<String> {
    match cesu8::from_java_cesu8(&b) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(NBTError::StringError)
    }
}