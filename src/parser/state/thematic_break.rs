use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{State, SubTransition, Transition};
use crate::unicode;

#[derive(Clone)]
pub struct ThematicBreakState {
    break_character: Option<char>,
    character_count: usize,
    leading_spaces: usize,
}

impl ThematicBreakState {
    pub fn new(character: Character) -> Self {
        if character.character() != unicode::SPACE {
            Self {
                break_character: Some(character.character()),
                character_count: 1,
                leading_spaces: 0,
            }
        } else {
            Self {
                break_character: None,
                character_count: 0,
                leading_spaces: 1,
            }
        }
    }

    fn is_break_character(character: char) -> bool {
        character == '-' || character == '_' || character == '*'
    }
}

impl Transition for ThematicBreakState {
    fn transition(self, character: Character) -> Action {
        match (self.break_character, character, self.leading_spaces) {
            // Case: follow-up break character
            (Some('-'), Character::Unescaped('-'), _) |
            (Some('_'), Character::Unescaped('_'), _) |
            (Some('*'), Character::Unescaped('*'), _) =>
                Action::Pass(State::ThematicBreak(Self {
                    character_count: self.character_count + 1,
                    ..self
                })),
            // Case: interrupting space
            (Some(_), Character::Unescaped(unicode::SPACE) | Character::Unescaped(unicode::TAB), _) =>
                Action::Pass(
                    State::ThematicBreak(self),
                ),
            // Case: first thematic break character
            (None, Character::Unescaped('-' | '_' | '*'), _) =>
                Action::Pass(
                    State::ThematicBreak(Self {
                        break_character: Some(character.character()),
                        character_count: 1,
                        ..self
                    })
                ),
            // Case: leading Space
            (None, Character::Unescaped(' '), 0..=2) =>
                Action::Pass(
                    State::ThematicBreak(Self {
                        leading_spaces: self.leading_spaces + 1,
                        ..self
                    })
                ),
            _ => Action::Dismiss,
        }
    }

    fn end(self) -> Action {
        if self.character_count >= 3 {
            Action::Complete(
                Block::Leaf(
                    Leaf::ThematicBreak
                )
            )
        } else {
            Action::Dismiss
        }
    }
}

impl SubTransition for ThematicBreakState {
    fn is_start(value: Character) -> bool {
        Self::is_break_character(value.character()) || value.character() == unicode::SPACE
    }
}