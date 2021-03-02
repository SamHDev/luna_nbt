use crate::tags::{Tag, TagIdent};
use crate::error::{NBTResult, NBTError, digest_io};

use byteorder::{BigEndian as BE, WriteBytesExt};
use std::io::Write;
use std::collections::HashMap;


pub(crate) fn write_tag<W: Write>(writer: &mut W, tag: &Tag) -> NBTResult<()>  {
    match tag {
        // Writing a Byte (i8)
        Tag::Byte(byte) => digest_io(writer.write_i8(*byte)),

        // Writing a Short (i16)
        Tag::Short(short) => digest_io(writer.write_i16::<BE>(*short)),

        // Writing a Int (i32)
        Tag::Int(int) => digest_io(writer.write_i32::<BE>(*int)),

        // Writing a Long(i64)
        Tag::Long(long) => digest_io(writer.write_i64::<BE>(*long)),

        // Writing a Float (f32)
        Tag::Float(float) => digest_io(writer.write_f32::<BE>(*float)),

        // Writing a Double (f64)
        Tag::Double(double) => digest_io(writer.write_f64::<BE>(*double)),

        // Writing an array of bytes (Vec<i8>)
        Tag::ByteArray(bytes) => {
            // Write length as a unsigned int. (4bytes)
            digest_io(writer.write_u32::<BE>(bytes.len() as u32))?;

            // Write items of array.
            for byte in bytes {
                digest_io(writer.write_i8(*byte))?;
            }
            Ok(())
        }

        // Write a string of utf-8 chars
        Tag::String(string) => write_string(writer, &string),

        Tag::List(list) => {
            // Check the list is valid (all items are of the same type) and return the type prefix.
            let list_type = ensure_list_integrity(&list)?;

            // Write type prefix.
            digest_io(writer.write_u8(list_type as u8))?;

            // Write List length
            digest_io(writer.write_u32::<BE>(list.len() as u32))?;

            // Write items (without prefix)
            for item in list {
                write_tag(writer, &item)?;
            }

            Ok(())
        }
        Tag::Compound(compound) => write_compound(writer, compound),
        Tag::IntArray(array) => {
            // Write length as a unsigned int. (4bytes)
            digest_io(writer.write_u32::<BE>(array.len() as u32))?;

            // Write items of array.
            for int in array {
                digest_io(writer.write_i32::<BE>(*int))?;
            }
            Ok(())
        }
        Tag::LongArray(array) => {
            // Write length as a unsigned int. (4bytes)
            digest_io(writer.write_u32::<BE>(array.len() as u32))?;

            // Write items of array.
            for long in array {
                digest_io(writer.write_i64::<BE>(*long))?;
            }
            Ok(())
        }
    }
}


// Function checks through items in a list to check if they are of the same type.
pub(crate) fn ensure_list_integrity(list: &Vec<Tag>) -> NBTResult<TagIdent> {
    // If list is empty, then type is TAG_End
    if list.len() == 0 {
        return Ok(TagIdent::TAG_End);
    }

    // Get first type.
    // Should be safe to unwrap here as we know there will be at least one element in the list.
    // We have ownership so it will never happen.
    let tag = list.get(0).unwrap().ident();

    // Loop through items
    for item in list {
        // Check
        if item.ident() != tag {
            // Error if user is bad at understanding nbt (like-me)
            return Err(NBTError::InvalidList { found: item.ident(), expecting: tag })
        }
    }

    Ok(tag)
}

// String writer.
// Strings are written the same way multiple times so this function exists.
pub(crate) fn write_string<W: Write>(writer: &mut W, string: &str) -> NBTResult<()> {
    // Get the UTF-8 bytes of the string
    let bytes = string.as_bytes();

    // Write length of string
    digest_io(writer.write_u16::<BE>(bytes.len() as u16))?;

    // Write the string.
    digest_io(writer.write_all(&bytes))
}

// Function for writing a root compound (implicit compound)
pub(crate) fn write_root<W: Write>(writer: &mut W, name: &str, elements: &HashMap<String, Tag>) -> NBTResult<()> {
    // Write implicit compound ident prefix.
    digest_io(writer.write_u8(TagIdent::TAG_Compound as u8))?;

    // Write root compound name
    write_string(writer, &name)?;

    // Write elements
    write_compound(writer, elements)
}

pub(crate) fn write_compound<W: Write>(writer: &mut W, compound: &HashMap<String, Tag>) -> NBTResult<()> {
    // Write items of compound
    for (name, payload) in compound {
        // Write element tag
        digest_io(writer.write_u8(payload.ident() as u8))?;

        // Write element name
        write_string(writer, &name)?;

        // write payload
        write_tag(writer, payload)?;
    }
    digest_io(writer.write_u8(TagIdent::TAG_End as u8))
}