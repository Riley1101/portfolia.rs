#![allow(dead_code)]

use serde::ser::{Serialize, SerializeStruct};

pub enum BlockType {
    Text(String),
    Code(String),
    Image(String),
    Quote(String),
    List(Vec<String>),
    Heading(String),
    Link(String),
    Html(String),
    Custom(String),
}

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BlockType", 2)?;
        let name = match self {
            BlockType::Text(_) => "text",
            BlockType::Code(_) => "code",
            BlockType::Image(_) => "image",
            BlockType::Quote(_) => "quote",
            BlockType::List(_) => "list",
            BlockType::Heading(_) => "heading",
            BlockType::Link(_) => "link",
            BlockType::Html(_) => "html",
            BlockType::Custom(_) => "custom",
        };
        state.serialize_field("name", name)?;
        state.end()
    }
}

pub struct Block {
    pub content: String,
    pub block_type: BlockType,
    pub id: i32,
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Block", 3)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("block_type", &self.block_type)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}
