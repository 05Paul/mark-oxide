use crate::error::Error;
use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::unicode;

pub const STAR: char = '*';
pub const DASH: char = '-';
pub const UNDERSCORE: char = '_';

#[derive(Clone, Copy, PartialEq)]
pub struct BreakCharacter(char);

impl BreakCharacter {
    pub const STAR: BreakCharacter = BreakCharacter(STAR);
    pub const DASH: BreakCharacter = BreakCharacter(DASH);
    pub const UNDERSCORE: BreakCharacter = BreakCharacter(UNDERSCORE);
}

impl TryFrom<&Character> for BreakCharacter {
    type Error = Error;

    fn try_from(value: &Character) -> Result<Self, Self::Error> {
        match value {
            Character::Unescaped(STAR) => Ok(BreakCharacter::STAR),
            Character::Unescaped(DASH) => Ok(BreakCharacter::DASH),
            Character::Unescaped(UNDERSCORE) => Ok(BreakCharacter::UNDERSCORE),
            _ => Err(Error::Conversion),
        }
    }
}

#[derive(Clone)]
pub struct ThematicBreakState {
    break_character: BreakCharacter,
    character_count: usize,
}

impl TryFrom<Character> for ThematicBreakState {
    type Error = Error;

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        Ok(
            Self {
                break_character: BreakCharacter::try_from(&value)?,
                character_count: 1,
            }
        )
    }
}

impl Transition for ThematicBreakState {
    fn transition(self, character: Character) -> Action {
        match (self.break_character, character) {
            // Case: follow-up break character
            (BreakCharacter::DASH, Character::Unescaped(DASH)) |
            (BreakCharacter::UNDERSCORE, Character::Unescaped(UNDERSCORE)) |
            (BreakCharacter::STAR, Character::Unescaped(STAR)) =>
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

    fn end_line(self, _: LineEnding) -> Action {
        self.end()
    }

    fn end(self) -> Action {
        if self.character_count >= 3 {
            Leaf::ThematicBreak.into_action()
        } else {
            Action::Dismiss
        }
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