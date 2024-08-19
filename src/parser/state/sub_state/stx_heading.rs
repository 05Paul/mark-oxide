use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::LineEnding;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

const LEVEL1: char = '=';
const LEVEL2: char = '-';

#[derive(Clone, Copy, PartialEq)]
pub struct UnderlineCharacter(char);

impl UnderlineCharacter {
    pub const LEVEL1: UnderlineCharacter = UnderlineCharacter(LEVEL1);
    pub const LEVEL2: UnderlineCharacter = UnderlineCharacter(LEVEL2);

    pub fn level(&self) -> usize {
        match self {
            &UnderlineCharacter::LEVEL1 => 1,
            &UnderlineCharacter::LEVEL2 => 2,
            _ => unreachable!()
        }
    }
}

impl TryFrom<&Character> for UnderlineCharacter {
    type Error = Error;

    fn try_from(value: &Character) -> Result<Self, Self::Error> {
        match value {
            Character::Unescaped(LEVEL1) => Ok(UnderlineCharacter::LEVEL1),
            Character::Unescaped(LEVEL2) => Ok(UnderlineCharacter::LEVEL2),
            _ => Err(Error::Conversion)
        }
    }
}

#[derive(Clone)]
pub struct STXHeadingState {
    content: String,
    underline: Option<UnderlineCharacter>,
    leading_spaces: usize,
    trail: bool,
    line_break: bool,
}

impl STXHeadingState {
    pub fn new(character: Character) -> Self {
        Self {
            content: character.to_string(),
            underline: None,
            leading_spaces: 0,
            trail: false,
            line_break: false,
        }
    }
}

impl Transition for STXHeadingState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (self.line_break, self.underline, character.clone(), self.leading_spaces, self.trail) {
            (_, None, Character::Unescaped(LEVEL1 | LEVEL2), _, _) => {
                self.underline = UnderlineCharacter::try_from(&character).ok();

                NonDeterministicTransitionEffect::pass(self)
            }
            (true, None, Character::Unescaped(unicode::SPACE), 0..=2, _) => {
                self.leading_spaces += 1;

                NonDeterministicTransitionEffect::pass(self)
            }
            (true, None, Character::Unescaped(unicode::SPACE), 3, _) => {
                NonDeterministicTransitionEffect::pass(self)
            }
            (_, None, character, _, _) => {
                self.content.push_str(&*character.to_string());

                NonDeterministicTransitionEffect::pass(self)
            }
            (true, Some(UnderlineCharacter::LEVEL1), Character::Unescaped(LEVEL1), _, false) |
            (true, Some(UnderlineCharacter::LEVEL2), Character::Unescaped(LEVEL2), _, false) =>
                NonDeterministicTransitionEffect::pass(self),
            (true, Some(_), Character::Unescaped(unicode::SPACE), _, _) => {
                self.trail = true;

                NonDeterministicTransitionEffect::pass(self)
            }
            _ => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        match (self.line_break, self.underline) {
            (true, Some(character)) => NonDeterministicTransitionEffect::complete::<Block>(
                Leaf::SetextHeading {
                    level: character.level(),
                    text: self.content.trim_end().to_string(),
                }
            ),
            (true, None) => NonDeterministicTransitionEffect::dismiss(),
            (false, _) => {
                self.content.push_str(&*line_ending.to_string());
                self.underline = None;
                self.leading_spaces = 0;
                self.trail = false;
                self.line_break = true;

                NonDeterministicTransitionEffect::pass(self)
            }
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match (self.line_break, self.underline) {
            (true, Some(character)) => Some(
                Leaf::SetextHeading {
                    level: character.level(),
                    text: self.content.trim_end().to_string(),
                }.into()
            ),
            _ => None,
        }
    }
}

impl From<STXHeadingState> for SubState {
    fn from(value: STXHeadingState) -> Self {
        SubState::STXHeading(value)
    }
}