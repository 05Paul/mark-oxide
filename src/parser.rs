use std::io;
use std::io::Read;

use crate::parser::character_parser::CharacterParser;
use crate::parser::document::Document;

mod state;
mod character;
mod document;
mod character_parser;
mod transition;
mod line_ending;
mod effect;

pub struct Parser<R> {
    reader: R,
    character_parser: CharacterParser,
}

impl<R: Sized + Read> Parser<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            character_parser: CharacterParser::new(),
        }
    }

    pub fn parse(mut self) -> io::Result<Document> {
        let mut data = String::new();
        self.reader.read_to_string(&mut data)?;

        for char in data.chars() {
            self.character_parser.parse_character(char);
        }

        Ok(self.character_parser.end_document())
    }

    pub fn parse_to_string(self) -> io::Result<String> {
        self.parse()
            .map(Document::to_string)
    }
}