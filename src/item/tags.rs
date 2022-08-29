use std::collections::{HashSet, HashMap, hash_map::DefaultHasher};
use std::num::ParseIntError;
use std::sync::Mutex;
use lazy_static::lazy_static;
use thiserror::Error;
use std::hash::{Hash, Hasher};
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod test {
    use super::Tag;
    use super::Tags;
    #[test]
    fn to_string() {
        let tag = Tag::new("Hi").unwrap();
        let name = tag.name();
        assert_eq!("Hi", name);
    }
    #[test]
    fn no_name() {
        let tag = Tag(0x64);
        let name = tag.name();
        assert_eq!(name, "0x64");
    }
    #[test]
    fn from_hex() {
        let tag = Tag::new("0x64").unwrap();
        assert_eq!(tag.name(), "0x64");
    }
    #[test]
    fn from_int() {
        let tag = Tag::new("69").unwrap();
        assert_eq!(tag.name(), "0x45");
    }
    #[test]
    fn serialize() {
        let mut tags = Tags::default();
        tags.insert(Tag::new("Hi").unwrap());
        tags.insert(Tag::new("Meta").unwrap());
        tags.insert(Tag::new("69").unwrap());
        tags.insert(Tag(0x64));
        let string = ron::to_string(&tags).unwrap();
        let tags: Tags = ron::from_str(&string).unwrap();
        assert!(tags.contains(&Tag::new("Hi").unwrap()));
        assert!(tags.contains(&Tag::new("Meta").unwrap()));
        assert!(tags.contains(&Tag::new("0x64").unwrap()));
        assert!(tags.contains(&Tag(69)));
    }
    #[test]
    fn deserialize() {
        let tags: Tags = ron::from_str(r#"["Meta","Hi","0x45","0x64"]"#).unwrap();
        assert!(tags.contains(&Tag::new("Hi").unwrap()));
        assert!(tags.contains(&Tag::new("Meta").unwrap()));
        assert!(tags.contains(&Tag::new("0x64").unwrap()));
        assert!(tags.contains(&Tag(69)));
    }
}

lazy_static! {
    static ref TAG_TO_STRING: Mutex<HashMap<Tag, String>> = Mutex::new(HashMap::default());
}

#[derive(Debug, Default)]
pub struct Tags(HashSet<Tag>);

impl Tags {
    pub fn insert(&mut self, tag: Tag) {
        self.0.insert(tag);
    }
    pub fn contains(&self, tag: &Tag) -> bool {
        self.0.contains(tag)
    }
    pub fn iter(&self) -> std::collections::hash_set::Iter<Tag> {
        self.0.iter()
    }
    pub fn merge(&mut self, other: &Tags) {
        for tag in other.iter() {
            self.0.insert(*tag);
        }
    }
}

impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for tag in self.0.iter() {
            seq.serialize_element(&tag.to_string())?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let data: Vec<String> = Vec::deserialize(deserializer)?;
        let mut set = HashSet::default();
        for tag in data {
            set.insert(Tag::from_str(&tag).unwrap());
        }
        Ok(Tags(set))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag(u64);

impl Tag {
    pub fn new(name: &str) -> Result<Tag, TagError> {
        Tag::from_str(name)
    }
    pub fn from_str(str: &str) -> Result<Tag, TagError> {
        let str = str.trim();
        if str.starts_with(|c: char| c.is_numeric()) {
            let id = match &str[..2] {
                "0x" | "0X" => {u64::from_str_radix(&str[2..], 16)?},
                _ => {str.parse()?},
            };
            Ok(Tag(id))
        } else {
            let mut hasher = DefaultHasher::default();
            str.hash(&mut hasher);
            let tag = Tag(hasher.finish());
            if let Ok(mut lock) = TAG_TO_STRING.lock() {
                lock.insert(tag, str.to_string());
                Ok(tag)
            } else {
                Err(TagError::LockFailed)
            }
        }
    }
    pub fn name(&self) -> String {
        match self.get_name() {
            Ok(name) => {name},
            Err(e) => {match e {
                TagError::LockFailed => {error!("Name Lookup is poisoned")},
                TagError::NoName => {warn!("no name registred for Tag({:#X})", self.0)},
                _ => unreachable!(),
            }; format!("{:#X}", self.0)}
        }
    }
    pub fn get_name(&self) -> Result<String, TagError> {
        let lock = if let Ok(lock) = TAG_TO_STRING.lock() {lock} else {return Err(TagError::LockFailed);};
        if let Some(name) = lock.get(self) {
            Ok(name.clone())
        } else {
            Err(TagError::NoName)
        }
    }
}

#[derive(Debug, Error)]
pub enum TagError {
    #[error("LockFailed")]
    LockFailed,
    #[error("No Regestered Name")]
    NoName,
    #[error("Parse Error")]
    ParseError(#[from] ParseIntError)
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.name()
    }
}