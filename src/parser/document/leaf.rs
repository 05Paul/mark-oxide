#[derive(Debug, Clone)]
pub enum Leaf {
    ThematicBreak,
    AtxHeading {
        level: usize,
        text: String,
    },
    SetextHeading {
        level: usize,
        text: String,
    },
    IndentedCodeBlock {
        text: String,
    },
}

impl Leaf {
    pub fn to_html(&self) -> String {
        match self {
            Leaf::ThematicBreak => "<hr />\n".into(),
            Leaf::AtxHeading { level, text } |
            Leaf::SetextHeading { level, text } => format!("<h{level}>{text}</h{level}>\n"),
            Leaf::IndentedCodeBlock { text } => format!("<pre><code>{text}\n</code></pre>\n"),
        }
    }
}