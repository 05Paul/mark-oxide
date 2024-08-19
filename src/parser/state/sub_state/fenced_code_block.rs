use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::LineEnding;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;
use crate::unicode::is_blank_text;

const BACKTICK: char = '`';
const TILDE: char = '~';

#[derive(Clone, Copy, PartialEq)]
pub struct FenceCharacter(char);

impl FenceCharacter {
    pub const BACKTICK: FenceCharacter = FenceCharacter(BACKTICK);
    pub const TILDE: FenceCharacter = FenceCharacter(TILDE);

    pub fn repeat(&self, n: usize) -> String {
        self.0.to_string().repeat(n)
    }
}

impl TryFrom<&Character> for FenceCharacter {
    type Error = Error;

    fn try_from(value: &Character) -> Result<Self, Self::Error> {
        match value {
            Character::Unescaped(BACKTICK) => Ok(FenceCharacter::BACKTICK),
            Character::Unescaped(TILDE) => Ok(FenceCharacter::TILDE),
            _ => Err(Error::Conversion),
        }
    }
}

#[derive(Clone)]
pub struct FencedCodeBlockState {
    text: String,
    indentation: usize,
    fence_character: FenceCharacter,
    opening_fence_length: usize,
    opening_fence_ended: bool,
    closing_fence_length: usize,
    leading_spaces: usize,
    non_leading: bool,
    info: Option<String>,
    info_done: bool,
}

impl FencedCodeBlockState {
    pub fn new(indentation: usize, character: Character) -> Result<Self, Error> {
        Ok(
            Self {
                text: "".into(),
                indentation,
                fence_character: FenceCharacter::try_from(&character)?,
                opening_fence_length: 1,
                opening_fence_ended: false,
                closing_fence_length: 0,
                leading_spaces: 0,
                non_leading: false,
                info: None,
                info_done: false,
            }
        )
    }
}

impl Transition for FencedCodeBlockState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (character, self.fence_character, self.opening_fence_ended, self.non_leading, self.leading_spaces, self.info_done) {
            (Character::Unescaped(BACKTICK), FenceCharacter::BACKTICK, false, _, _, false) |
            (Character::Unescaped(TILDE), FenceCharacter::TILDE, false, _, _, false) => {
                self.opening_fence_length += 1;

                NonDeterministicTransitionEffect::pass(self)
            }
            (Character::Unescaped(unicode::SPACE), _, _, _, _, false) => {
                self.info_done = self.info.is_some();

                NonDeterministicTransitionEffect::pass(self)
            }
            (Character::Unescaped(character) | Character::Escaped(character), _, _, _, _, false) => {
                if let Some(info) = &mut self.info {
                    info.push_str(&*character.to_string())
                } else {
                    self.info = Some(character.to_string())
                }

                NonDeterministicTransitionEffect::pass(self)
            }
            (Character::Unescaped(_) | Character::Escaped(_), _, false, _, _, true) => NonDeterministicTransitionEffect::pass(self),
            (Character::Unescaped(BACKTICK), FenceCharacter::BACKTICK, true, false, 0..=3, true) |
            (Character::Unescaped(TILDE), FenceCharacter::TILDE, true, false, 0..=3, true) => {
                self.closing_fence_length += 1;

                NonDeterministicTransitionEffect::pass(self)
            }
            (Character::Unescaped(unicode::SPACE), _, true, false, _, true) => {
                self.text.push_str(&*self.fence_character.repeat(self.closing_fence_length));
                self.closing_fence_length = 0;
                self.leading_spaces += 1;
                self.non_leading = self.closing_fence_length != 0;

                NonDeterministicTransitionEffect::pass(self)
            }
            (character, _, true, _, _, true) => {
                self.text.push_str(&*unicode::SPACE.to_string().repeat(
                    self.leading_spaces.checked_sub(self.indentation).unwrap_or(0)
                ));
                self.text.push_str(&*self.fence_character.repeat(self.closing_fence_length));
                self.text.push_str(&*character.to_raw_string());

                self.closing_fence_length = 0;
                self.leading_spaces = 0;
                self.non_leading = true;

                NonDeterministicTransitionEffect::pass(self)
            }
            _ => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.non_leading = false;
        self.info_done = true;

        if self.opening_fence_length < 3 {
            NonDeterministicTransitionEffect::dismiss()
        } else if !self.opening_fence_ended {
            self.leading_spaces = 0;
            self.opening_fence_ended = true;

            NonDeterministicTransitionEffect::pass(self)
        } else if self.closing_fence_length >= self.opening_fence_length {
            NonDeterministicTransitionEffect::complete::<Block>(
                Leaf::FencedCodeBlock {
                    text: self.text.clone(),
                    info: self.info.clone(),
                }
            )
        } else {
            self.text.push_str(&*unicode::SPACE.to_string().repeat(self.leading_spaces.checked_sub(self.indentation).unwrap_or(0)));
            self.text.push_str(&*self.fence_character.repeat(self.closing_fence_length));
            self.text.push_str(&*line_ending.to_string());
            self.leading_spaces = 0;
            self.closing_fence_length = 0;

            NonDeterministicTransitionEffect::pass(self)
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        let mut text = self.text;

        if !ends_with_blank_or_new_line(&text) {
            text.push(unicode::LINE_FEED);
        }

        // text.push_str(self.fence_character.to_string().repeat(self.closing_fence_length).as_str());


        if self.opening_fence_length < 3 {
            None
        } else {
            Some(
                Leaf::FencedCodeBlock {
                    text,
                    info: self.info,
                }.into()
            )
        }
    }
}

impl From<FencedCodeBlockState> for SubState {
    fn from(value: FencedCodeBlockState) -> Self {
        SubState::FencedCodeBlock(value)
    }
}

fn ends_with_blank_or_new_line(text: &str) -> bool {
    text.ends_with(|character| character == unicode::LINE_FEED || character == unicode::CARRIAGE_RETURN) ||
        text.lines()
            .last()
            .map(is_blank_text)
            .unwrap_or(true)
}
