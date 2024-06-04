use std::io;
use std::io::Read;
use crate::state::State;

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

    pub fn parse_to_string(self) -> io::Result<String> {
        Ok("".to_string())
    }
}

struct Document;

impl Document {
    fn new() -> Self {
        Self
    }
}