use crate::error::Error;
use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
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
    fn transition(self, character: Character) -> Action {
        match (character, self.fence_character, self.opening_fence_ended, self.non_leading, self.leading_spaces, self.info_done) {
            (Character::Unescaped(BACKTICK), FenceCharacter::BACKTICK, false, _, _, false) |
            (Character::Unescaped(TILDE), FenceCharacter::TILDE, false, _, _, false) => Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        opening_fence_length: self.opening_fence_length + 1,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(unicode::SPACE), _, _, _, _, false) => {
                Action::Pass(
                    State::FencedCodeBlock(
                        Self {
                            info_done: self.info.is_some(),
                            ..self
                        }
                    )
                )
            }
            (Character::Unescaped(character) | Character::Escaped(character), _, _, _, _, false) => Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        info: self.info.map(|info| info + character.to_string().as_str()).or(Some(character.to_string())),
                        ..self
                    }
                )
            ),
            (Character::Unescaped(_) | Character::Escaped(_), _, false, _, _, true) => Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        ..self
                    }
                )
            ),
            (Character::Unescaped(BACKTICK), FenceCharacter::BACKTICK, true, false, 0..=3, true) |
            (Character::Unescaped(TILDE), FenceCharacter::TILDE, true, false, 0..=3, true) => Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        closing_fence_length: self.closing_fence_length + 1,
                        ..self
                    }
                )
            ),
            (Character::Unescaped(unicode::SPACE), _, true, false, _, true) => {
                Action::Pass(
                    State::FencedCodeBlock(
                        Self {
                            text: self.text + self.fence_character.repeat(self.closing_fence_length).as_str(),
                            closing_fence_length: 0,
                            leading_spaces: self.leading_spaces + 1,
                            non_leading: self.closing_fence_length != 0,
                            ..self
                        }
                    )
                )
            }
            (Character::Unescaped(character) | Character::Escaped(character), _, true, _, _, true) => Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        text: self.text + unicode::SPACE.to_string().repeat(self.leading_spaces.checked_sub(self.indentation).unwrap_or(0)).as_str() + self.fence_character.repeat(self.closing_fence_length).as_str() + character.to_string().as_str(),
                        closing_fence_length: 0,
                        leading_spaces: 0,
                        non_leading: true,
                        ..self
                    }
                )
            ),
            _ => Action::Dismiss,
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Action {
        if self.opening_fence_length < 3 {
            Action::Dismiss
        } else if !self.opening_fence_ended {
            Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        opening_fence_ended: true,
                        leading_spaces: 0,
                        non_leading: false,
                        info_done: true,
                        ..self
                    }
                )
            )
        } else if self.closing_fence_length >= self.opening_fence_length {
            Leaf::FencedCodeBlock {
                text: self.text,
                info: self.info,
            }.into_action()
        } else {
            Action::Pass(
                State::FencedCodeBlock(
                    Self {
                        text: self.text + unicode::SPACE.to_string().repeat(self.leading_spaces.checked_sub(self.indentation).unwrap_or(0)).as_str() + self.fence_character.repeat(self.closing_fence_length).as_str() + <LineEnding as Into<String>>::into(line_ending).as_str(),
                        closing_fence_length: 0,
                        leading_spaces: 0,
                        non_leading: false,
                        info_done: true,
                        ..self
                    }
                )
            )
        }
    }

    fn end(self) -> Action {
        let mut text = self.text;

        if !ends_with_blank_or_new_line(&text) {
            text.push(unicode::LINE_FEED);
        }

        // text.push_str(self.fence_character.to_string().repeat(self.closing_fence_length).as_str());


        if self.opening_fence_length < 3 {
            Action::Dismiss
        } else {
            Leaf::FencedCodeBlock {
                text,
                info: self.info,
            }.into_action()
        }
    }
}

impl SubTransition for FencedCodeBlockState {
    fn is_start(value: Character) -> bool {
        FencedCodeBlockState::new(0, value).is_ok()
    }
}

fn ends_with_blank_or_new_line(text: &str) -> bool {
    text.ends_with(|character| character == unicode::LINE_FEED || character == unicode::CARRIAGE_RETURN) ||
        text.lines()
            .last()
            .map(is_blank_text)
            .unwrap_or(true)
}