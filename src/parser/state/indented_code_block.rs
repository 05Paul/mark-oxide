use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::unicode;

#[derive(Clone)]
pub struct IndentedCodeBlockState {
    text: String,
    line_break_buffer: String,
    leading_space_count: usize,
    leading_spaces: bool,
    newline: bool,
}

impl IndentedCodeBlockState {
    pub fn new(leading_internal_spaces: usize) -> Self {
        Self {
            text: "".repeat(leading_internal_spaces).into(),
            line_break_buffer: "".into(),
            leading_space_count: 4,
            leading_spaces: false,
            newline: true,
        }
    }
}

impl Transition for IndentedCodeBlockState {
    fn transition(self, character: Character) -> Action {
        match (character.clone(), self.newline, self.leading_spaces, self.leading_space_count) {
            (Character::Unescaped(unicode::SPACE), _, _, 0..=3) => Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        leading_space_count: self.leading_space_count + 1,
                        newline: false,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(unicode::SPACE), _, true, 4..) => Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        line_break_buffer: self.line_break_buffer + character.to_string().as_str(),
                        leading_space_count: self.leading_space_count + 1,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(unicode::TAB), _, _, 0..=3) => Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        leading_space_count: self.leading_space_count + 4,
                        newline: false,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(unicode::TAB), _, true, 4..) => Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        line_break_buffer: self.line_break_buffer + character.to_string().as_str(),
                        leading_space_count: self.leading_space_count + 4,
                        ..self
                    }
                )
            ),
            (character, _, _, 4..) => Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        text: self.text + self.line_break_buffer.as_str() + character.to_raw_string().as_str(),
                        line_break_buffer: "".into(),
                        newline: false,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(_), _, _, 0..=3) => Leaf::IndentedCodeBlock {
                text: self.text.trim_end().to_string()
            }.into_action()
                .merge(State::default().transition(character)),
            _ => Action::Dismiss
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Action {
        if self.text.trim().is_empty() {
            return Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        text: String::new(),
                        line_break_buffer: "".into(),
                        leading_space_count: 0,
                        leading_spaces: true,
                        newline: true,
                    }
                )
            );
        }


        if self.leading_spaces {
            return Action::Pass(
                State::IndentedCodeBlock(
                    Self {
                        text: self.text,
                        line_break_buffer: self.line_break_buffer + <LineEnding as Into<String>>::into(line_ending).as_str(),
                        leading_space_count: 0,
                        leading_spaces: true,
                        newline: true,
                    }
                )
            );
        }

        Action::Pass(
            State::IndentedCodeBlock(
                Self {
                    text: self.text + <LineEnding as Into<String>>::into(line_ending).as_str(),
                    line_break_buffer: "".into(),
                    leading_space_count: 0,
                    leading_spaces: true,
                    newline: true,
                }
            )
        )
    }

    fn end(self) -> Action {
        Leaf::IndentedCodeBlock {
            text: self.text.trim_end_matches([
                unicode::CARRIAGE_RETURN,
                unicode::LINE_FEED,
            ]).to_string()
        }.into_action()
    }
}

impl SubTransition for IndentedCodeBlockState {
    fn is_start(_: Character) -> bool {
        true
    }
}