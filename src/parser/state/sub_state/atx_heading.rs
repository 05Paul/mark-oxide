use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::LineEnding;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

pub const HASHTAG: char = '#';

#[derive(Clone)]
pub struct ATXHeadingState {
    pub character_count: usize,
    pub text: String,
    temp: String,
}

impl Transition for ATXHeadingState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (character.clone(), self.character_count) {
            // Case: hashtag
            (Character::Unescaped(HASHTAG), 0..=5) => {
                let length = self.text.len() + self.temp.len();
                let last_temp = self.temp.chars().last();
                let contains_hashtag = self.temp.contains(HASHTAG);

                match (length, last_temp, contains_hashtag) {
                    // Case: trailing whitespace
                    (1.., Some(unicode::SPACE | unicode::TAB), true) => {
                        self.text.push_str(&*self.temp);
                        self.temp = character.to_string();

                        NonDeterministicTransitionEffect::pass(self)
                    }
                    // Case: trailing whitespace or hashtag
                    (1.., Some(unicode::SPACE | unicode::TAB | HASHTAG), _) => {
                        self.temp.push_str(&*character.to_string());

                        NonDeterministicTransitionEffect::pass(self)
                    }
                    // Case: trailing character
                    (1.., _, _) => {
                        self.text.push_str(&*character.to_string());

                        NonDeterministicTransitionEffect::pass(self)
                    }
                    // Case: content character
                    _ => {
                        self.character_count += 1;

                        NonDeterministicTransitionEffect::pass(self)
                    }
                }
            }
            // Case: non-leading space
            (Character::Unescaped(unicode::SPACE), 1..) => {
                self.temp.push_str(&*character.to_string());

                NonDeterministicTransitionEffect::pass(self)
            }
            // Case: tab
            (Character::Unescaped(unicode::TAB), 1..) => {
                self.temp.push_str(&*character.to_string());

                NonDeterministicTransitionEffect::pass(self)
            }
            // Case: non whitespace character after first '#'
            (_, 1..) => {
                self.text.push_str(&*self.temp);
                self.text.push_str(&*character.to_string());
                self.temp = "".into();

                NonDeterministicTransitionEffect::pass(self)
            }
            // Case: dismiss
            _ => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        match self.character_count {
            0 => NonDeterministicTransitionEffect::dismiss(),
            _ => NonDeterministicTransitionEffect::complete::<Block>(
                Leaf::AtxHeading {
                    level: self.character_count,
                    text: self.text.trim().to_string(),
                }
            ),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self.character_count {
            0 => None,
            _ => Some(
                Leaf::AtxHeading {
                    level: self.character_count,
                    text: self.text.trim().to_string(),
                }.into()
            ),
        }
    }
}

impl TryFrom<Character> for ATXHeadingState {
    type Error = Error;

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        if let Character::Unescaped(HASHTAG) = value {
            Ok(
                Self {
                    character_count: 1,
                    text: "".to_string(),
                    temp: "".to_string(),
                }
            )
        } else {
            Err(Error::StartState)
        }
    }
}

impl From<ATXHeadingState> for SubState {
    fn from(value: ATXHeadingState) -> Self {
        SubState::ATXHeading(value)
    }
}