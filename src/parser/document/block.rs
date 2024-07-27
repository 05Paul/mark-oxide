use crate::parser::document::leaf::Leaf;

#[derive(Debug, Clone)]
pub enum Block {
    Container,
    Leaf(Leaf),
}

impl Block {
    pub fn to_html(&self) -> String {
        match self {
            Block::Container => String::new(),
            Block::Leaf(leaf) => leaf.to_html(),
        }
    }
}