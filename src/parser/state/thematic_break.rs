use crate::error::Error;
use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::unicode;

pub const STAR: char = '*';
pub const DASH: char = '-';
pub const UNDERSCORE: char = '_';

#[derive(Clone)]
pub struct ThematicBreakState {
    break_character: char,
    character_count: usize,
}

impl TryFrom<Character> for ThematicBreakState {
    type Error = Error;

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        if Self::is_start(value) {
            Ok(
                Self {
                    break_character: value.character(),
                    character_count: 1,
                }
            )
        } else {
            Err(Error::StartStateError)
        }
    }
}

impl Transition for ThematicBreakState {
    fn transition(self, character: Character) -> Action {
        match (self.break_character, character) {
            // Case: follow-up break character
            (DASH, Character::Unescaped(DASH)) |
            (UNDERSCORE, Character::Unescaped(UNDERSCORE)) |
            (STAR, Character::Unescaped(STAR)) =>
                Action::Pass(State::ThematicBreak(Self {
                    character_count: self.character_count + 1,
                    ..self
                })),
            // Case: interrupting space
            (_, Character::Unescaped(unicode::SPACE | unicode::TAB)) =>
                Action::Pass(
                    State::ThematicBreak(self),
                ),
            _ => Action::Dismiss,
        }
    }

    fn end(self) -> Action {
        if self.character_count >= 3 {
            Leaf::ThematicBreak.into_action()
        } else {
            Action::Dismiss
        }
    }

    fn end_line(self, _: LineEnding) -> Action {
        self.end()
    }
}

impl SubTransition for ThematicBreakState {
    fn is_start(value: Character) -> bool {
        match value {
            Character::Unescaped(DASH | UNDERSCORE | STAR) => true,
            _ => false
        }
    }
}