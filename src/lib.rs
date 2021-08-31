//! # Named Binary Tag (NBT)
//! The The Named Binary Tag, is a structured binary format used by the game Minecraft for a variety
//! of purposes such as, Player Data and World Saves as well as being used within the Minecraft Protocol.
//!
//! [NBT Specification](https://wiki.vg/NBT#Specification)
//!
//! ## This Crate
//! This crate is __yet__ another implementation of the NBT format.
//!
//! ### Key features
//! - Support for Serialisation and Deserialization with the [Serde](https://serde.rs) framework.
//! - Ability to create partial or complete documents through the `Tag` and `Blob` objects.
//! - Ability to read/write from a socket or buffer.
//!
//! ### Cargo Features
//! - `serde`             (default) includes Serde serialisation and deserialization support.
//! - `serde_boolean`     (default) converts booleans to bytes during serialisation and deserialization.
//! - `serde_unsigned`    converts unsigned to their signed counterparts during serialisation and deserialization.
//! - `debug`             (default) debug for tags
//! - `arrays`            utils for writing byte, int and long arrays. (dev branch)
//! - `compression`       gzip and DEFLATE support. (dev branch)
//!
//! ### Operation
//! This crate has two seperate operations that allow data to be mutated.
//! 1. Read/Writing - `Tags/Blobs` `<--NBT-->` `Bytes/Buffer`
//! 2. Encoding/Decoding - `Tags/Blobs` `<--Serde-->` `Structs`
//!
//! ## Quick Start
//!
//! #### Tags
//! One way of creating partial NBT objects is with tags.
//! ```
//! use nbt::Tag;
//!
//! // An example of a TAG_Byte with value 42.
//! let byte = Tag::Byte(42);
//!
//! // An example of a TAG_String with value "Hello!"
//! let string = Tag::String("Hello!".to_string());
//!
//! // An example of TAG_List containing bytes, with values of [1,2,3]
//! let list = Tag::List(vec![Tag::Byte(1), Tag::Byte(2), Tag::Byte(3)]);
//!
//! // An example of a compound
//! use std::collections::HashMap;
//! let mut map = HashMap::<String, Tag>::new();
//! map.insert("age".to_string(), Tag::Byte(18));
//! map.insert("id".to_string(), Tag::Int(69420));
//! let compound = Tag::Compound(map);
//! ```
//! #### Blobs
//! Blobs allow for you to create full NBT objects/documents.
//! ```
//! use nbt::{Blob, Tag};
//!
//! // Creating a blob
//! let mut blob = Blob::new();
//!
//! // Inserting a tag
//! blob.insert("age", Tag::Byte(18));
//!
//! // Using the ToTag trait to insert a tag.
//! blob.insert("id", 69420_i32);
//! ```
//! Blobs reprsent the root compound, and hence can be named.
//!
//! When a name is not specifed it defaults to `""`
//! ```
//! use nbt::{Blob};
//!
//! // Creating a blob with a name
//! let mut blob = Blob::create("user");
//! blob.insert("name", "pokechu22");
//! ```
//!
//! #### Encoding / Writing
//! You can encode a partial or full NBT object using the `NBTWrite` trait.
//! ```
//! use nbt::{Tag, NBTWrite};
//! let tag = Tag::Float(69.420);
//!
//! // Writing to a buffer
//! let mut buffer = Vec::new();
//! tag.write(&mut buffer).unwrap();
//!
//! // Outputting to a Vec.
//! let bytes = tag.bytes().unwrap();
//!
//! println!("{:?}", bytes);
//! ```
//!
//! #### Decoding / Reading
//! ```
//! use nbt::{Tag, NBTWrite, NBTRead};
//! let data = /* vec![...] */
//! # vec![10, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 8, 0, 4, 110, 97, 109, 101, 0, 9, 66, 97, 110, 97, 110, 114, 97, 109, 97, 0];
//!
//! // Reading from a buffer
//! use std::io::Cursor;
//! let mut cursor = Cursor::new(data.clone());
//! let one = Tag::read(&mut cursor).unwrap();
//! // Also works for: Blob::read(...)
//!
//! // Reading from vec
//! let two = Tag::from_bytes(data);
//! // Also works for: Blob::from_bytes(...)
//! ```
//!
//! ### Serde
//! This library has full serde serialisation and deserialization support for all types in the
//! [serde data model](https://serde.rs/data-model.html) except for u8 (byte)arrays.
//!
//! Serde support requires the `with_serde` cargo feature, which is enabled by default.
//!
//! #### Encoding
//! Here is an basic example for encoding between a struct and bytes
//! ```rust
//! use serde::Serialize;
//! use nbt::{encode, NBTWrite};
//!
//! // Define a Serializable struct
//! #[derive(Serialize)]
//! pub struct HelloWorld {
//!     name: String
//! }
//!
//! // Instantiate
//! let hello = HelloWorld {
//!     name: "Dinnerbone".to_string()
//! };
//!
//! // Encode/Serialise the struct into a blob.
//! let blob = encode(&hello).unwrap();
//!
//! // get the bytes from the blob.
//! let bytes = blob.bytes().unwrap();
//!
//! assert_eq!(bytes, vec![10, 0, 0, 8, 0, 4, 110, 97, 109, 101, 0, 10, 68, 105, 110, 110, 101, 114, 98, 111, 110, 101, 0])
//! ```
//!
//! #### Decoding
//! Here is the reverse operation for the above example
//! ```
//! use serde::Deserialize;
//! use nbt::{Blob, NBTRead, decode};
//!
//! // Define a Deserialisable struct
//! #[derive(Deserialize, PartialEq, Debug)]
//! pub struct HelloWorld {
//!     name: String
//! }
//!
//! // Bytes
//! let bytes = vec![10, 0, 0, 8, 0, 4, 110, 97, 109, 101, 0, 10, 68, 105, 110, 110, 101, 114, 98, 111, 110, 101, 0];
//!
//! // Create a blob from bytes
//! let blob = Blob::from_bytes(bytes).unwrap();
//!
//! // Deserialize the blob into the struct
//! let hello = decode::<HelloWorld>(blob).unwrap();
//!
//! assert_eq!(hello, HelloWorld {
//!     name: "Dinnerbone".to_string()
//! });
//! ```
//!
//! #### Serde Functions
//! - `BLOB --> SERDE` [`decode(...)`](crate::decode())
//! - `BLOB <-- SERDE` [`encode(...)`](crate::encode())
//! - `TAG --> SERDE` [`decode_tag(...)`](crate::decode_tag)
//! - `TAG <-- SERDE` [`encode_tag(...)`](crate::encode_tag)
//! - `BLOB --> SERDE + NAME` [`decode_named(...)`](crate::decode_named)
//! - `BLOB <-- SERDE + NAME` [`encode_named(...)`](crate::encode_named)

pub(crate) mod tags;
pub(crate) mod error;
pub(crate) mod blob;
pub(crate) mod encode;
pub(crate) mod decode;
pub(crate) mod front;
pub(crate) mod util;
pub(crate) mod compound;
// pub(crate) mod map;

pub use util::{FromTag, ToTag};
pub use front::{NBTWrite, NBTRead};
pub use tags::{TagIdent, Tag};
pub use blob::Blob;
pub use compound::Compound;


#[cfg(test)]
pub mod tests;

#[cfg(feature= "serde")]
mod ser;
#[cfg(feature= "serde")]
mod de;

// mod list;


#[cfg(feature= "serde")]
pub use front::{encode, encode_named, encode_tag, decode, decode_named, decode_tag};