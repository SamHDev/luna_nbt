use crate::{Tag, ToTag, FromTag};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub trait Map: Sized {
    fn _inner_insert(&mut self, key: String, item: Tag) -> Option<Tag>;
    fn _inner_remove(&mut self, key: &str) -> Option<Tag>;
    fn _inner_get(&self, key: &str) -> Option<&Tag>;
    fn _inner_iter(&self) -> std::collections::hash_map::Iter<String, Tag>;
    fn _inner_into_map(self) -> HashMap<String, Tag>;
    fn _inner_from_map(s: HashMap<String, Tag>) -> Self;

    fn insert_tag(&mut self, key: &str, tag: Tag) -> Option<Tag> {
        self._inner_insert(key.to_string(), tag)
    }
    fn insert<T: ToTag>(&mut self, key: &str, tag: T) -> Option<Tag> {
        self._inner_insert(key.to_string(), tag.into_tag())
    }

    fn remove_tag(&mut self, key: &str) -> Option<Tag> {
        self._inner_remove(key)
    }
    fn remove<T: FromTag>(&mut self, key: &str) -> Option<T> {
        T::from_tag(self._inner_remove(key)?)
    }

    fn get_tag(&self, key: &str) -> Option<&Tag> {
        self._inner_get(key)
    }
    fn get<T: FromTag>(&self, key: &str) -> Option<&Tag> {
        T::from_borrowed_tag(self._inner_get(key)?)
    }

    fn tags(&self) -> std::collections::hash_map::Iter<String, Tag> {
        self._inner_iter()
    }

    fn from<T: ToTag>(map: HashMap<String, T>) -> Self {
        Self::_inner_from_map(
            map.into_iter().map(|(k, v)| (k, v.into_tag())).collect()
        )
    }

    fn map(self) -> HashMap<String, Tag> {
        self._inner_into_map()
    }
}


impl Map for HashMap<String, Tag> {
    fn _inner_insert(&mut self, key: String, item: Tag) -> Option<Tag> {
        HashMap::insert(&mut self, key, item)
    }

    fn _inner_remove(&mut self, key: &str) -> Option<Tag> {
        HashMap::remove(&mut self, key)
    }

    fn _inner_get(&self, key: &str) -> Option<&Tag> {
        HashMap::get(&self, key)
    }

    fn _inner_iter(&self) -> std::collections::hash_map::Iter<String, Tag> {
        self.iter()
    }

    fn _inner_into_map(self) -> HashMap<String, Tag> {
        self
    }

    fn _inner_from_map(s: HashMap<String, Tag>) -> Self {
        s
    }
}