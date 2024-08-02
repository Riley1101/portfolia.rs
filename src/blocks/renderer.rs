#![allow(dead_code)]
use super::contents::{Block, BlockType};

pub enum BlockResult {
    Invalid,
    Html(String),
}

pub fn render_paragraph(block: Block) {
    let Block {
        content,
        block_type,
        ..
    } = block;
    let _ = match block_type {
        BlockType::Text => BlockResult::Html(format!("<p>{}</p>", content)),
        _ => BlockResult::Invalid,
    };
    println!("{} {}", block_type, content);
}
