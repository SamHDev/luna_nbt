pub(crate) mod tags;
pub(crate) mod error;
pub(crate) mod blob;
pub(crate) mod encode;
pub(crate) mod decode;
pub(crate) mod front;
pub mod util;

pub use front::{NBTWrite, NBTRead};
pub use blob::Blob;
pub use tags::{TagIdent, Tag};

#[cfg(test)]
pub mod tests;

#[cfg(feature="with_serde")]
mod ser;
#[cfg(feature="with_serde")]
mod de;

#[cfg(feature="with_serde")]
pub use front::{encode, encode_named, encode_tag};