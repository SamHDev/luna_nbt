use crate::tags::Tag;
use std::ops::Deref;
use std::collections::HashMap;
use crate::util::{ToTag, FromTag};

#[cfg_attr(feature="debug", derive(Debug))]
/// A NBT Document containing an implicit compound and root name.
///
/// ## Example
/// ```
/// # use nbt::{Blob, NBTWrite, NBTRead};
///
/// // Creation
/// let mut blob = Blob::create("hello world");
/// blob.insert("name", "Bananrama");
///
/// // Encoding
/// let bytes = blob.bytes().unwrap();
///
/// // Decoding
/// let decoded = Blob::from_bytes(bytes).unwrap();
///
/// // Retrieval
/// assert_eq!(decoded.get::<String>("name").unwrap(), &("Bananrama".to_string()))
/// ```
///
pub struct Blob {
    /// Name of the root compound
    pub root: String,
    /// Elements of the root compound
    pub elements: HashMap<String, Tag>
}

impl Blob {
    /// Create a new `Blob` with a given root compound name.
    pub fn create(root: &str) -> Blob {
        Blob { root: root.to_string() , elements: HashMap::new() }
    }

    /// Create a new `Blob` with a empty root name.
    pub fn new() -> Blob {
        Blob { root: String::new() , elements: HashMap::new() }
    }

    /// Insert a element into the root compound.
    ///
    /// The payload element takes a `Tag` or any type that implements `ToTag`
    /// ```
    /// # use nbt::{Blob, Tag};
    /// # let mut blob = Blob::new();
    /// blob.insert("name", "Hello World");
    /// blob.insert("name", Tag::Byte(127));
    /// ```
    pub fn insert<P: ToTag>(&mut self, name: &str, payload: P) -> Option<Tag> {
        self.elements.insert(name.to_string(), payload.into_tag())
    }

    /// Get a element from the root compound, with a given type.
    ///
    /// Uses the `FromTag` trait to convert a tag into a desired type.
    /// ```
    /// # use nbt::Blob;
    /// # let mut blob = Blob::new();
    /// # blob.insert("name", "Hello World");
    /// let name = blob.get::<String>("name"); // Some("Hello World")
    /// let none = blob.get::<i8>("name"); // None
    /// # assert_eq!(name.unwrap(), &("Hello World".to_string()));
    /// # assert_eq!(none, None)
    /// ```
    pub fn get<T: FromTag>(&self, name: &str) -> Option<&T> where Self: Sized {
        T::from_borrowed_tag(self.elements.get(&name.to_string())?.clone())
    }

    /// Get the NBT blob as a compound tag.
    pub fn compound(self) -> Tag {
        Tag::Compound(self.elements)
    }
}

impl Deref for Blob {
    type Target = HashMap<String, Tag>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}
