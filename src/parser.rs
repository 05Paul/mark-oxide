use std::io;
use std::io::Read;
use crate::state::{Action, Character, State, Transition};
use crate::unicode;

pub struct Parser<R> {
    reader: R,
    state: State,
    buffer: Vec<char>,
    document: Document,
}

impl<R: Sized + Read> Parser<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            state: State::default(),
            buffer: Vec::new(),
            document: Document::new(),
        }
    }

    pub fn parse_to_string(mut self) -> io::Result<String> {
        let mut data = String::new();
        self.reader.read_to_string(&mut data)?;

        for char in data.chars() {
            let char = if char == unicode::NULL {
                unicode::REPLACEMENT
            } else {
                char
            };

            let (new_state, action) = self.state.transition(Character::new(char));
            self.state = new_state;

            self.handle_action(action);
        }

        let (new_state, action) = self.state.end();
        self.state = new_state;
        self.handle_action(action);

        Ok(self.document.to_string())
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Complete(state) => self.handle_completion(state),
            _ => {}
        }
    }

    fn handle_completion(&mut self, state: State) {
        match state {
            State::ThematicBreak(_) => self.document.push(
                Block::Leaf(
                    Leaf::ThematicBreak
                )
            ),
            _ => {}
        }
    }
}

struct Document {
    content: Vec<Block>,
}

impl Document {
    fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    fn push(&mut self, block: Block) {
        self.content.push(block);
    }

    fn to_string(self) -> String {
        let mut out = String::new();

        for block in self.content {
            out.extend(block.to_html().chars());
        }

        out
    }
}

#[derive(Debug)]
enum Block {
    Container,
    Leaf(Leaf),
}

impl Block {
    fn to_html(&self) -> String {
        match self {
            Block::Container => String::new(),
            Block::Leaf(leaf) => leaf.to_html(),
        }
    }
}

#[derive(Debug)]
enum Leaf {
    ThematicBreak,
}

impl Leaf {
    fn to_html(&self) -> String {
        match self {
            Leaf::ThematicBreak => "<hr />\n".into()
        }
    }
}