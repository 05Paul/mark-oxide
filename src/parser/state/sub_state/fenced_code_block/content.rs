use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::closing::ClosingState;
use crate::parser::transition::{Transition, TransitionEffect};

/// # Fenced Code Block: Content
/// Code block content
///
/// ## Transition
/// * [NewlineState] on LineEnding
///
/// ## Completion
/// * DocumentEnding
pub struct ContentState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
    pub(crate) info_string: Option<String>,
    pub(crate) content: String,
}

impl Transition for ContentState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        self.content.push_str(&*character.to_raw_string());
        NonDeterministicTransitionEffect::pass(self)
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.content.push_str(&*line_ending.to_string());
        NonDeterministicTransitionEffect::transition_into::<NewlineState>(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        Some(Leaf::FencedCodeBlock {
            text: self.content,
            info: self.info_string,
        }.into())
    }
}

impl From<NewlineState> for ContentState {
    fn from(value: NewlineState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
            content: value.content,
        }
    }
}

impl From<ClosingState> for ContentState {
    fn from(value: ClosingState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
            content: value.content,
        }
    }
}

impl From<ContentState> for FencedCodeBlockSubState {
    fn from(value: ContentState) -> Self {
        FencedCodeBlockSubState::Content(value)
    }
}