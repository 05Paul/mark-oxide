use crate::parser::document::block::Block;

pub mod block;
pub mod leaf;

pub struct Document {
    content: Vec<Block>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn push(&mut self, block: Block) {
        self.content.push(block);
    }

    pub fn to_string(self) -> String {
        let mut out = String::new();

        for block in self.content {
            out.extend(block.to_html().chars());
        }

        out
    }
}