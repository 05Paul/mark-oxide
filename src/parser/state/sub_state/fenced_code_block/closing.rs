use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::content::ContentState;
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Fenced Code Block: Closing Fence
/// Code block closing fence
///
/// ## Transition
/// * [ContentState] on non-whitespace (after closing fence is long enough) character other
/// than required [FenceCharacter]
///
/// ## Completion
/// * DocumentEnding
/// * LineEnding when [FenceCharacter] count > indentation
pub struct ClosingState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
    pub(crate) info_string: Option<String>,
    pub(crate) content: String,
    leading_spaces: usize,
    closing_fence_length: usize,
    trailing: String,
}

impl ClosingState {
    fn finish(&mut self, string: String) {
        let leading_spaces = self.leading_spaces.checked_sub(self.indentation)
            .unwrap_or(0);
        self.content.push_str(&*unicode::SPACE.to_string().repeat(leading_spaces));
        self.content.push_str(&*self.character.repeat(self.closing_fence_length));
        self.content.push_str(&*self.trailing);
        self.content.push_str(&*string);
    }
}

impl Transition for ClosingState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            Character::Unescaped(fenced_code_block::BACKTICK | fenced_code_block::TILDE) => {
                if let Ok(character) = FenceCharacter::try_from(&character) {
                    if character == self.character {
                        self.closing_fence_length += 1;
                        NonDeterministicTransitionEffect::pass(self)
                    } else {
                        self.finish(character.character().to_string());
                        NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
                    }
                } else {
                    self.finish(character.to_raw_string());
                    NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
                }
            }
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
               if self.closing_fence_length >= self.character_count {
                   self.trailing.push_str(&*character.to_raw_string());
                   NonDeterministicTransitionEffect::pass(self)
               } else {
                   self.finish(character.to_string());
                   NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
               }
            }
            character => {
                self.finish(character.to_raw_string());
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
            }
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        if self.closing_fence_length >= self.character_count {
            NonDeterministicTransitionEffect::complete::<Block>(Leaf::FencedCodeBlock {
                text: self.content,
                info: self.info_string,
            })
        } else {
            self.finish(line_ending.to_string());
            NonDeterministicTransitionEffect::transition_into::<NewlineState>(self)
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        Some(Leaf::FencedCodeBlock {
            text: self.content,
            info: self.info_string,
        }.into())
    }
}

impl From<NewlineState> for ClosingState {
    fn from(value: NewlineState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
            content: value.content,
            leading_spaces: value.leading_spaces,
            closing_fence_length: 1,
            trailing: "".to_string(),
        }
    }
}

impl From<ClosingState> for FencedCodeBlockSubState {
    fn from(value: ClosingState) -> Self {
        FencedCodeBlockSubState::Closing(value)
    }
}