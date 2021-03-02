use crate::tags::Tag;
use std::ops::Deref;
use std::collections::HashMap;
use crate::util::{ToTag, FromTag};

#[derive(Debug)]
pub struct Blob {
    pub root: String,
    pub elements: HashMap<String, Tag>
}

impl Blob {
    pub fn create(root: &str) -> Blob {
        Blob { root: root.to_string() , elements: HashMap::new() }
    }
    pub fn new() -> Blob {
        Blob { root: String::new() , elements: HashMap::new() }
    }

    pub fn insert<P: ToTag>(&mut self, name: &str, payload: P) -> Option<Tag> {
        self.elements.insert(name.to_string(), payload.into_tag())
    }

    pub fn get<T: FromTag>(&self, name: &str) -> Option<&T> where Self: Sized {
        T::from_borrowed_tag(self.elements.get(&name.to_string())?.clone())
    }

}

impl Deref for Blob {
    type Target = HashMap<String, Tag>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}
