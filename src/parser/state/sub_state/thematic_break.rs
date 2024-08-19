use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::LineEnding;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
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

impl Transition for ThematicBreakState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (self.break_character, character) {
            // Case: follow-up break character
            (BreakCharacter::DASH, Character::Unescaped(DASH)) |
            (BreakCharacter::UNDERSCORE, Character::Unescaped(UNDERSCORE)) |
            (BreakCharacter::STAR, Character::Unescaped(STAR)) => {
                self.character_count += 1;

                NonDeterministicTransitionEffect::pass(self)
            },
            // Case: interrupting space
            (_, Character::Unescaped(unicode::SPACE | unicode::TAB)) =>
                NonDeterministicTransitionEffect::pass(self),
            _ => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        if self.character_count >= 3 {
            NonDeterministicTransitionEffect::complete::<Block>(Leaf::ThematicBreak)
        } else {
            NonDeterministicTransitionEffect::dismiss()
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        if self.character_count >= 3 {
            Some(Leaf::ThematicBreak.into())
        } else {
            None
        }
    }
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

impl From<ThematicBreakState> for SubState {
    fn from(value: ThematicBreakState) -> Self {
        SubState::ThematicBreak(value)
    }
}