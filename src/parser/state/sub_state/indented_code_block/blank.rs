use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::indented_code_block::content::ContentState;
use crate::parser::state::sub_state::indented_code_block::IndentedCodeBlockSubState;
use crate::parser::state::sub_state::indented_code_block::newline::NewlineState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Indented Code Block: Blank Line
/// Potential blank line
///
/// ## Transition
/// * [ContentState] on non-blank character
/// * [NewLineState] on LineEnding
///
/// ## Completion
/// * DocumentEnding
///
/// ## Dismissal
/// * DocumentEnding when content is empty
/// * LineEnding when content is empty
pub struct BlankLineState {
    pub(crate) content: String,
}

impl Transition for BlankLineState {
    type Effect = NonDeterministicTransitionEffect<IndentedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.content.push_str(&*character.to_raw_string());
                NonDeterministicTransitionEffect::pass(self)
            }
            character => {
                self.content.push_str(&*character.to_raw_string());
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
            }
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        if self.content.is_empty() {
            return NonDeterministicTransitionEffect::dismiss();
        }

        self.content.push_str(&*line_ending.to_string());
        NonDeterministicTransitionEffect::transition_into::<NewlineState>(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        IndentedCodeBlockSubState::complete(self)
            .end()
    }
}

impl Default for BlankLineState {
    fn default() -> Self {
        Self {
            content: "".to_string(),
        }
    }
}

impl From<NewlineState> for BlankLineState {
    fn from(value: NewlineState) -> Self {
        Self {
            content: value.content,
        }
    }
}

impl From<BlankLineState> for IndentedCodeBlockSubState {
    fn from(value: BlankLineState) -> Self {
        IndentedCodeBlockSubState::Blank(value)
    }
}
