#![allow(dead_code)]

use serde::ser::{Serialize, SerializeStruct};
use uuid::Uuid;

pub enum Mark {
    H1,
    H2,
    H3,
    H4,
    Bold,
    Italic,
}

impl Serialize for Mark {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Mark", 2)?;
        let mark = match self {
            Mark::H1 => "h1",
            Mark::H2 => "h2",
            Mark::H3 => "h3",
            Mark::H4 => "h4",
            Mark::Bold => "b",
            Mark::Italic => "i",
        };
        state.serialize_field("name", mark)?;
        state.end()
    }
}

pub enum BlockType {
    Text,
    Code,
    Image,
    Quote,
    Heading,
    Link,
    Html,
}

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BlockType", 2)?;
        let name = match self {
            BlockType::Text => "text",
            BlockType::Code => "code",
            BlockType::Image => "image",
            BlockType::Quote => "quote",
            BlockType::Heading => "heading",
            BlockType::Link => "link",
            BlockType::Html => "html",
        };
        state.serialize_field("name", name)?;
        state.end()
    }
}

pub struct Block {
    pub mark: Option<Mark>,
    pub content: String,
    pub block_type: BlockType,
    pub id: String,
}

impl Block {
    pub fn new(content: String, block_type: BlockType) -> Self {
        Self {
            mark: None,
            content,
            block_type,
            id: format!("{}", Uuid::new_v4()),
        }
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 3)?;
        state.serialize_field("mark", &self.mark)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("block_type", &self.block_type)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}
