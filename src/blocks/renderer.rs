#![allow(dead_code)]
use std::fmt::Display;

use super::contents::{Block, BlockType, Mark};

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

pub fn mark_renderer(block: Option<Mark>) -> String {
    match block {
        Some(Mark::Bold) => "strong".to_string(),
        Some(Mark::Italic) => "em".to_string(),
        Some(Mark::H1) => "h1".to_string(),
        Some(Mark::H2) => "h2".to_string(),
        Some(Mark::H3) => "h3".to_string(),
        Some(Mark::H4) => "h4".to_string(),
        Some(Mark::Normal) => "p".to_string(),
        _ => "div".to_string(),
    }
}

pub fn render_block(block: Block) -> BlockResult {
    let Block {
        content,
        block_type,
        mark,
        ..
    } = block;
    match block_type {
        BlockType::Text => {
            let syntax = mark_renderer(mark);
            BlockResult::Html(format!("<{syntax}>{content}</{syntax}>"))
        }
        BlockType::Code => BlockResult::Html(format!("<pre><code>{content}</code></pre>")),
        BlockType::Image => BlockResult::Html(format!("<img src='{content}' alt='image' />")),
        BlockType::Quote => BlockResult::Html(format!("<blockquote>{content}</blockquote>")),
        BlockType::Link => BlockResult::Html(format!("<a href='{content}'>{content}</a>")),
        _ => BlockResult::Invalid,
    }
}
