#![allow(dead_code)]
use std::fmt::Display;

use super::contents::{Block, BlockType};

pub enum BlockResult {
    Invalid,
    Html(String),
}

impl Display for BlockResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockResult::Invalid => write!(f, "Invalid"),
            BlockResult::Html(html) => write!(f, "{}", html),
        }
    }
}

pub fn render_paragraph(block: Block) -> BlockResult {
    let Block {
        content,
        block_type,
        ..
    } = block;
    match block_type {
        BlockType::Text => BlockResult::Html(format!("<p>{}</p>", content)),
        _ => BlockResult::Invalid,
    }
}
