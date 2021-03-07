use crate::{Tag, ToTag, FromTag};

pub struct List<T> {
    items: Vec<T>
}

impl<T> List<T> {
    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.items.insert(index, item)
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from(list: Vec<T>) -> Self {
        Self { items: list }
    }

    pub fn list(self) -> Vec<T> {
        self.items
    }
}

impl<T: ToTag> List<T> {
    pub fn to_tags(self) -> Vec<Tag> {
        self.items.into_iter().map(|x| x.into_tag()).collect()
    }
}

impl<T: FromTag> List<T> {
    pub fn from_tags(tags: Vec<Tag>) -> Self {
        Self {
            items: tags.into_iter()
                .map(|x| T::from_tag(x))
                .filter(|x| x.is_some()).collect()
        }
    }
}

impl<T: FromTag> List<&T> {
    pub fn from_borrowed_tags(tags: &Vec<Tag>) -> Self {
        Self {
            items: tags.into_iter()
                .map(|x| T::from_borrowed_tag(x))
                .filter(|x| x.is_some()).collect()
        }
    }
}

impl<T: ToTag> ToTag for List<T> {
    fn into_tag(self) -> Tag {
        Tag::List(self.to_tags())
    }
}

impl<T: FromTag> FromTag for List<T> {
    fn from_tag(tag: Tag) -> Option<Self> {
        if let Tag::List(list) = tag {
            Some(Self::from_tags(list))
        } else {
            None
        }
    }

    fn from_borrowed_tag(tag: &Tag) -> Option<List<&T>> {
        if let Tag::List(list) = tag {
            Some(List::from_borrowed_tags(list))
        } else {
            None
        }
    }
}