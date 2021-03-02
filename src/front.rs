use std::io::{Write, Read, Cursor};
use crate::error::{NBTResult, NBTError};
use crate::tags::Tag;
use crate::encode::{write_tag, write_root};
use crate::blob::Blob;
use crate::decode::{read_tag, read_ident, read_root};
use serde::Serialize;
use crate::ser::NBTSerializer;
use crate::TagIdent;
use serde::de::DeserializeOwned;
use crate::de::NBTDeserializer;

/// A trait supporting encoding of NBT Tags/Blobs into bytes.
pub trait NBTWrite {
    fn write<W: Write>(&self, writer: &mut W) -> NBTResult<()>;

    fn bytes(&self) -> NBTResult<Vec<u8>> {
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

/// A trait supporting decoding of bytes into NBT/Tags.
///
/// This trait provides two functions:
/// - `read` for reading from a readable source or buffer
/// - `from_bytes` for reading from a array of bytes
///
///
pub trait NBTRead: Sized {
    /// Function for reading from a buffer.
    fn read<R: Read>(reader: &mut R) -> NBTResult<Self>;

    /// Function for reading from a byte array.
    fn from_bytes<B: AsRef<[u8]>>(data: B) -> NBTResult<Self> {
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
/// Encode a Serde serializable value into a NBT Tag.
///
/// ### Example
/// ```
/// use nbt::{encode_tag, Tag};
///
/// let list: Vec<i8> = vec![127, 42, 10];
/// let tag = encode_tag(&list).unwrap().unwrap();
///
/// # assert_eq!(tag, Tag::List(vec![Tag::Byte(127), Tag::Byte(42), Tag::Byte(10)]));
/// ```
pub fn encode_tag<T: Serialize>(o: &T) -> NBTResult<Option<Tag>> {
    o.serialize(NBTSerializer)
}

/// Encode a Serde serializable value into a NBT Blob with a given root name.
///
/// ### Example
/// ```
/// use nbt::{encode_tag, encode_named, Tag};
/// use std::collections::HashMap;
/// use serde::Serialize;
///
/// // Define a Serializable Struct
/// #[derive(Serialize)]
/// pub struct Example {
///     name: String,
/// }
///
/// // Create a instance
/// let example = Example {
///     name: "Bananrama".to_string(),
/// };
/// // Encode a NBT blob with name "hello_world"
/// let tag = encode_named(&example, "hello_world").unwrap();
///
/// # let mut test = HashMap::new();
/// # test.insert("name".to_string(), Tag::String("Bananrama".to_string()));
/// # assert_eq!(tag.compound(), Tag::Compound(test));
/// ```
///
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
/// Encode a Serde serializable value into a NBT Blob with a empty root name.
///
/// Encode a Serde serializable value into a NBT Blob with a given root name.
/// Encode a Serde serializable value into a NBT Blob with a given root name.
///
/// ### Example
/// ```
/// use nbt::{encode_tag, encode, Tag};
/// use std::collections::HashMap;
/// use serde::Serialize;
///
/// // Define a Serializable Struct
/// #[derive(Serialize)]
/// pub struct Example {
///     foo: String,
///     bar: i8,
///     baz: i16
/// }
///
/// // Create a instance
/// let example = Example {
///     foo: "Hello World!".to_string(),
///     bar: 42,
///     baz: 25565
/// };
/// // Encode a NBT blob with name "example"
/// let tag = encode(&example).unwrap();
///
/// # let mut test = HashMap::new();
/// # test.insert("foo".to_string(), Tag::String("Hello World!".to_string()));
/// # test.insert("bar".to_string(), Tag::Byte(42));
/// # test.insert("baz".to_string(), Tag::Short(25565));
/// # assert_eq!(tag.compound(), Tag::Compound(test));
/// ```
///
pub fn encode<T: Serialize>(o: &T) -> NBTResult<Blob> {
    encode_named(o, "")
}

#[cfg(feature="with_serde")]
/// Decode a NBT Tag into a Serde deserializable value.
///
/// ### Example
/// ```
/// use nbt::{Tag, decode_tag};
///
/// // Create a Byte List.
/// let tag = Tag::List(vec![Tag::Byte(127), Tag::Byte(42)]);
///
/// // Decode the tag into a vec.
/// let list: Vec<i8> = decode_tag(tag).unwrap();
///
/// assert_eq!(list, vec![127, 42]);
/// ```
pub fn decode_tag<T: DeserializeOwned>(tag: Tag) -> NBTResult<T> {
    T::deserialize(NBTDeserializer::some(tag))
}


#[cfg(feature="with_serde")]
/// Decode a NBT Blob into a Serde deserializable value.
///
/// ### Example
/// ```
/// use nbt::{Tag, decode, Blob};
/// use serde::Deserialize;
///
/// // Create Deserializable struct
/// #[derive(Deserialize, PartialEq, Debug)]
/// pub struct Example {
///     foo: String
/// }
///
/// // Create a Blob
/// let mut blob = Blob::new();
/// blob.insert("foo", "bar");
///
/// // Decode the data from blob
/// let data: Example = decode(blob).unwrap();
///
/// assert_eq!(data, Example { foo: "bar".to_string() });
/// ```
pub fn decode<T: DeserializeOwned>(tag: Blob) -> NBTResult<T> {
    T::deserialize(NBTDeserializer::some(Tag::Compound(tag.elements)))
}

#[cfg(feature="with_serde")]
/// Decode a NBT Blob into a Serde deserializable value and the given root name.
///
/// ### Example
/// ```
/// use nbt::{Tag, Blob, decode_named};
/// use serde::Deserialize;
///
/// // Create Deserializable struct
/// #[derive(Deserialize, PartialEq, Debug)]
/// pub struct Example {
///     foo: String
/// }
///
/// // Create a Blob
/// let mut blob = Blob::create("baz");
/// blob.insert("foo", "bar");
///
/// // Decode the root name and data from blob
/// let (root, data): (String, Example) = decode_named(blob).unwrap();
///
/// assert_eq!(data, Example { foo: "bar".to_string() });
/// assert_eq!(root, "baz".to_string());
/// ```
pub fn decode_named<T: DeserializeOwned>(tag: Blob) -> NBTResult<(String, T)> {
    Ok((tag.root.clone(), T::deserialize(NBTDeserializer::some(Tag::Compound(tag.elements)))?))
}

