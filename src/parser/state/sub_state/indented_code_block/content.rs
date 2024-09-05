use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::indented_code_block::blank::BlankLineState;
use crate::parser::state::sub_state::indented_code_block::IndentedCodeBlockSubState;
use crate::parser::state::sub_state::indented_code_block::newline::NewlineState;
use crate::parser::transition::{Transition, TransitionEffect};

/// # Indented Code Block: Content
/// Code block content
///
/// ## Transition
/// * [NewlineState] on LineEnding
///
/// ## Completion
/// * DocumentEnding
pub struct ContentState {
    pub(crate) content: String,
}

impl Transition for ContentState {
    type Effect = NonDeterministicTransitionEffect<IndentedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        self.content.push_str(&*character.to_raw_string());
        NonDeterministicTransitionEffect::pass(self)
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.content.push_str(&*line_ending.to_string());
        NonDeterministicTransitionEffect::transition_into::<NewlineState>(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        IndentedCodeBlockSubState::complete(self)
            .end()
    }
}

impl From<BlankLineState> for ContentState {
    fn from(value: BlankLineState) -> Self {
        Self {
            content: value.content,
        }
    }
}

impl From<ContentState> for IndentedCodeBlockSubState {
    fn from(value: ContentState) -> Self {
        IndentedCodeBlockSubState::Content(value)
    }
}