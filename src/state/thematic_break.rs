use crate::state::{Action, Character, State, SubTransition, Transition};
use crate::state::default::DefaultState;
use crate::unicode;

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
    fn transition(self, character: Character) -> (State, Action) {
        match (self.break_character, character, self.leading_spaces) {
            // Case: follow-up break character
            (Some('-'), Character::Unescaped('-'), _) |
            (Some('_'), Character::Unescaped('_'), _) |
            (Some('*'), Character::Unescaped('*'), _) => (
                State::ThematicBreak(Self {
                    character_count: self.character_count + 1,
                    ..self
                }),
                Action::Pass,
            ),
            // Case: interrupting space
            (Some(_), Character::Unescaped(unicode::SPACE) | Character::Unescaped(unicode::TAB), _) => (
                State::ThematicBreak(self),
                Action::Pass,
            ),
            // Case: first thematic break character
            (None, Character::Unescaped('-' | '_' | '*'), _) => (
                State::ThematicBreak(Self {
                    break_character: Some(character.character()),
                    character_count: 1,
                    ..self
                }),
                Action::Pass,
            ),
            // Case: leading Space
            (None, Character::Unescaped(' '), 0..=2) => (
                State::ThematicBreak(Self {
                    leading_spaces: self.leading_spaces + 1,
                    ..self
                }),
                Action::Pass,
            ),
            _ => (
                State::Default(DefaultState),
                Action::Dismiss,
            )
        }
    }

    fn end(self) -> (State, Action) {
        if self.character_count >= 3 {
            (
                State::Default(DefaultState),
                Action::Complete(State::ThematicBreak(self)),
            )
        } else {
            (
                State::Default(DefaultState),
                Action::Dismiss,
            )
        }
    }
}

impl SubTransition for ThematicBreakState {
    fn is_start(value: Character) -> bool {
        Self::is_break_character(value.character()) || value.character() == unicode::SPACE
    }
}