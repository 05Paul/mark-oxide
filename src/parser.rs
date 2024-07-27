mod state;
mod character;
mod action;
mod document;

use std::io;
use std::io::Read;
use std::ops::Deref;
use crate::parser::action::Action;
use crate::parser::document::Document;
use crate::parser::state::{State, Transition};
use crate::unicode;

pub struct Parser<R> {
    reader: R,
    state: State,
    document: Document,
}

impl<R: Sized + Read> Parser<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            state: State::default(),
            document: Document::new(),
        }
    }

    pub fn parse_to_string(mut self) -> io::Result<String> {
        let mut data = String::new();
        self.reader.read_to_string(&mut data)?;

        for char in data.chars() {
            let char = Self::replace_null(char);

            let action = self.state.transition(char.into());
            match action {
                Action::Pass(state) => self.state = state,
                Action::Dismiss => self.state = State::default(),
                Action::Complete(block) => {
                    self.state = State::default();
                    self.document.push(block);
                }
                Action::Bi { first, second } => match (first.deref().clone(), second.deref().clone()) {
                    (Action::Complete(block), Action::Pass(state)) => {
                        self.state = state;
                        self.document.push(block)
                    },
                    _ => {unreachable!()}
                },
            }
        }

        let action = self.state.end();
        match action {
            Action::Pass(state) => self.state = state,
            Action::Dismiss => self.state = State::default(),
            Action::Complete(block) => {
                self.state = State::default();
                self.document.push(block);
            }
            Action::Bi { .. } => unreachable!(),
        }

        Ok(self.document.to_string())
    }

    fn replace_null(character: char) -> char {
        if character == unicode::NULL {
            unicode::REPLACEMENT
        } else {
            character
        }
    }
}