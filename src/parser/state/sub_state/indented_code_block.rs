mod content;
mod newline;
mod blank;

use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::{LineEnding, State};
use crate::parser::state::handler::Handler;
use crate::parser::state::sub_state::indented_code_block::blank::BlankLineState;
use crate::parser::state::sub_state::indented_code_block::content::ContentState;
use crate::parser::state::sub_state::indented_code_block::newline::NewlineState;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

pub type IndentedCodeBlockState = Handler<SubState, IndentedCodeBlockSubState>;

impl Default for IndentedCodeBlockState {
    fn default() -> Self {
        IndentedCodeBlockState::new(
            IndentedCodeBlockSubState::default()
        )
    }
}

impl From<IndentedCodeBlockState> for SubState {
    fn from(value: IndentedCodeBlockState) -> Self {
        value.state().into()
    }
}

pub enum IndentedCodeBlockSubState {
    Content(ContentState),
    NewLine(NewlineState),
    Blank(BlankLineState),
    Complete(State),
}

impl IndentedCodeBlockSubState {
    pub fn complete(state: impl Into<IndentedCodeBlockSubState>) -> NonDeterministicTransitionEffect<IndentedCodeBlockSubState, Option<Block>> {
        NonDeterministicTransitionEffect::complete::<Block>(Leaf::IndentedCodeBlock {
            text: state.into().content().trim_end_matches([
                unicode::CARRIAGE_RETURN,
                unicode::LINE_FEED,
            ]).to_string(),
        })
    }

    fn content(self) -> String {
        match self {
            IndentedCodeBlockSubState::Content(state) => state.content,
            IndentedCodeBlockSubState::NewLine(state) => state.content,
            IndentedCodeBlockSubState::Blank(state) => state.content,
            IndentedCodeBlockSubState::Complete(_) => unreachable!(),
        }
    }
}

impl Transition for IndentedCodeBlockSubState {
    type Effect = NonDeterministicTransitionEffect<IndentedCodeBlockSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            IndentedCodeBlockSubState::Content(state) => state.transition(character),
            IndentedCodeBlockSubState::NewLine(state) => state.transition(character),
            IndentedCodeBlockSubState::Blank(state) => state.transition(character),
            IndentedCodeBlockSubState::Complete(_) => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            IndentedCodeBlockSubState::Content(state) => state.end_line(line_ending),
            IndentedCodeBlockSubState::NewLine(state) => state.end_line(line_ending),
            IndentedCodeBlockSubState::Blank(state) => state.end_line(line_ending),
            IndentedCodeBlockSubState::Complete(_) => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            IndentedCodeBlockSubState::Content(state) => state.end(),
            IndentedCodeBlockSubState::NewLine(state) => state.end(),
            IndentedCodeBlockSubState::Blank(state) => state.end(),
            IndentedCodeBlockSubState::Complete(_) => None,
        }
    }
}


impl Default for IndentedCodeBlockSubState {
    fn default() -> Self {
        IndentedCodeBlockSubState::Blank(BlankLineState::default())
    }
}

impl From<IndentedCodeBlockSubState> for SubState {
    fn from(value: IndentedCodeBlockSubState) -> Self {
        match value {
            IndentedCodeBlockSubState::Complete(state) => SubState::IntoSuper(state),
            state => SubState::IndentedCodeBlock(
                IndentedCodeBlockState::new(state)
            ),
        }
    }
}