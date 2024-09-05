use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::indented_code_block::blank::BlankLineState;
use crate::parser::state::sub_state::indented_code_block::content::ContentState;
use crate::parser::state::sub_state::indented_code_block::IndentedCodeBlockSubState;
use crate::parser::state::State;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Indented Code Block: Newline
/// Code block newline
///
/// ## Transition
/// * [BlankLineState] after an equivalent of 4 spaces
///
/// ## Completion
/// * non-whitespace character
/// * DocumentEnding
pub struct NewlineState {
    pub(crate) content: String,
    leading_spaces: usize,
}

impl Transition for NewlineState {
    type Effect = NonDeterministicTransitionEffect<IndentedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        self.leading_spaces += character.space_count();

        if self.leading_spaces >= 4 {
            NonDeterministicTransitionEffect::transition_into::<BlankLineState>(self)
        } else if character.is_blank() {
            NonDeterministicTransitionEffect::pass(self)
        } else {
            let (state, _) = State::from_leading_space_count(self.leading_spaces)
                .transition(character)
                .content();

            NonDeterministicTransitionEffect::new(
                IndentedCodeBlockSubState::Complete(state),
                Some(Leaf::IndentedCodeBlock {
                    text: self.content.trim_end_matches([
                        unicode::CARRIAGE_RETURN,
                        unicode::LINE_FEED,
                    ]).to_string(),
                }.into()),
            )
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.content.push_str(&*line_ending.to_string());
        self.leading_spaces = 0;
        NonDeterministicTransitionEffect::pass(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        IndentedCodeBlockSubState::complete(self)
            .end()
    }
}

impl From<ContentState> for NewlineState {
    fn from(state: ContentState) -> Self {
        Self {
            content: state.content,
            leading_spaces: 0,
        }
    }
}

impl From<BlankLineState> for NewlineState {
    fn from(value: BlankLineState) -> Self {
        Self {
            content: value.content,
            leading_spaces: 0,
        }
    }
}

impl From<NewlineState> for IndentedCodeBlockSubState {
    fn from(value: NewlineState) -> Self {
        IndentedCodeBlockSubState::NewLine(value)
    }
}