use crate::tags::Tag;
use std::collections::HashMap;

pub trait ToTag {
    fn into_tag(self) -> Tag;
}

pub trait FromTag: Sized {
    fn from_tag(tag: Tag) -> Option<Self>;
    fn from_borrowed_tag(tag: &Tag) -> Option<&Self>;
}


impl ToTag for i8 { fn into_tag(self) -> Tag { Tag::Byte(self) } }
impl ToTag for i16 { fn into_tag(self) -> Tag { Tag::Short(self) } }
impl ToTag for i32 { fn into_tag(self) -> Tag { Tag::Int(self) } }
impl ToTag for i64 { fn into_tag(self) -> Tag { Tag::Long(self) } }
impl ToTag for f32 { fn into_tag(self) -> Tag { Tag::Float(self) } }
impl ToTag for f64 { fn into_tag(self) -> Tag { Tag::Double(self) } }
impl ToTag for String { fn into_tag(self) -> Tag { Tag::String(self) } }
impl ToTag for &str { fn into_tag(self) -> Tag { Tag::String(self.to_string()) } }

impl ToTag for Vec<i8> { fn into_tag(self) -> Tag { Tag::ByteArray(self) }}
impl ToTag for Vec<i32> { fn into_tag(self) -> Tag { Tag::IntArray(self) }}
impl ToTag for Vec<i64> { fn into_tag(self) -> Tag { Tag::LongArray(self) }}

//impl ToTag for Vec<i16> { fn into_tag(self) -> Tag { Tag::List(self.into_iter().map(|x| x.into_tag()).collect()) } }
impl<T: ToTag> ToTag for HashMap<String, T> { fn into_tag(self) -> Tag { Tag::Compound(self.into_iter().map(|(k, v)| (k, v.into_tag())).collect()) } }

impl FromTag for i8 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Byte(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Byte(v) = tag { Some(v) } else { None } } }
impl FromTag for i16 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Short(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Short(v) = tag { Some(v) } else { None } } }
impl FromTag for i32 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Int(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Int(v) = tag { Some(v) } else { None } } }
impl FromTag for i64 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Long(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Long(v) = tag { Some(v) } else { None } } }
impl FromTag for f32 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Float(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Float(v) = tag { Some(v) } else { None } } }
impl FromTag for f64 { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::Double(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::Double(v) = tag { Some(v) } else { None } } }
impl FromTag for String { fn from_tag(tag: Tag) -> Option<Self> { if let Tag::String(v) = tag { Some(v) } else { None } } fn from_borrowed_tag(tag: &Tag) -> Option<&Self> { if let Tag::String(v) = tag { Some(v) } else { None } } }
