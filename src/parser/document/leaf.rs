#[derive(Debug, Clone)]
pub enum Leaf {
    ThematicBreak,
    AtxHeading {
        level: usize,
        text: String,
    },
}

impl Leaf {
    pub fn to_html(&self) -> String {
        match self {
            Leaf::ThematicBreak => "<hr />\n".into(),
            Leaf::AtxHeading { level, text } => format!("<h{level}>{text}</h{level}>\n"),
        }
    }
}