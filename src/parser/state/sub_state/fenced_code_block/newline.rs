use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::closing::ClosingState;
use crate::parser::state::sub_state::fenced_code_block::content::ContentState;
use crate::parser::state::sub_state::fenced_code_block::info::InfoStringState;
use crate::parser::state::sub_state::fenced_code_block::opening::OpeningState;
use crate::parser::state::sub_state::fenced_code_block::opening_trail::OpeningTrailingState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Fenced Code Block: Newline
/// Code block newline
///
/// ## Transition
/// * [ContentState] on whitespace character after 3 whitespace characters
/// * [ContentState] on non-whitespace character
/// * [ClosingState] on matching [FenceCharacter]
///
/// ## Completion
/// * DocumentEnding
pub struct NewlineState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
    pub(crate) info_string: Option<String>,
    pub(crate) content: String,
    pub(crate) leading_spaces: usize,
}

impl Transition for NewlineState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.leading_spaces += character.space_count();
                if self.leading_spaces >= 4 {
                    self.content.push_str(&*leading_spaces_string(self.leading_spaces, self.indentation));
                    NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
                } else {
                    NonDeterministicTransitionEffect::pass(self)
                }
            }
            character @ Character::Unescaped(fenced_code_block::BACKTICK | fenced_code_block::TILDE) => {
                if let Ok(character) = FenceCharacter::try_from(&character) {
                    if character == self.character {
                        NonDeterministicTransitionEffect::transition_into::<ClosingState>(self)
                    } else {
                        self.content.push_str(&*leading_spaces_string(self.leading_spaces, self.indentation));
                        self.content.push(character.character());
                        NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
                    }
                } else {
                    self.content.push_str(&*leading_spaces_string(self.leading_spaces, self.indentation));
                    self.content.push_str(&*character.to_raw_string());
                    NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
                }
            }
            character => {
                self.content.push_str(&*leading_spaces_string(self.leading_spaces, self.indentation));
                self.content.push_str(&*character.to_raw_string());
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
            }
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.content.push_str(&*leading_spaces_string(self.leading_spaces, self.indentation));
        self.leading_spaces = 0;
        self.content.push_str(&*line_ending.to_string());
        NonDeterministicTransitionEffect::pass(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        Some(Leaf::FencedCodeBlock {
            text: self.content,
            info: self.info_string,
        }.into())
    }
}

impl From<OpeningState> for NewlineState {
    fn from(value: OpeningState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: None,
            content: "".to_string(),
            leading_spaces: 0,
        }
    }
}

impl From<InfoStringState> for NewlineState {
    fn from(value: InfoStringState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: Some(value.info_string),
            content: "".to_string(),
            leading_spaces: 0,
        }
    }
}

impl From<OpeningTrailingState> for NewlineState {
    fn from(value: OpeningTrailingState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: Some(value.info_string),
            content: "".to_string(),
            leading_spaces: 0,
        }
    }
}

impl From<ContentState> for NewlineState {
    fn from(value: ContentState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
            content: value.content,
            leading_spaces: 0,
        }
    }
}

impl From<ClosingState> for NewlineState {
    fn from(value: ClosingState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
            content: value.content,
            leading_spaces: 0,
        }
    }
}

impl From<NewlineState> for FencedCodeBlockSubState {
    fn from(value: NewlineState) -> Self {
        FencedCodeBlockSubState::Newline(value)
    }
}

pub fn leading_spaces_string(leading_spaces: usize, indentation: usize) -> String {
    let leading_spaces = leading_spaces.checked_sub(indentation)
        .unwrap_or(0);
    unicode::SPACE.to_string().repeat(leading_spaces)
}