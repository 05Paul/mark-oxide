use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::ATXHeadingState;
use crate::parser::state::sub_state::fenced_code_block::FencedCodeBlockState;
use crate::parser::state::sub_state::indented_code_block::IndentedCodeBlockState;
use crate::parser::state::sub_state::stx_heading::{STXHeadingState};
use crate::parser::state::sub_state::thematic_break::ThematicBreakState;
use crate::parser::state::State;
use crate::parser::transition::{Transition, TransitionEffect};

pub(crate) mod fenced_code_block;
pub(crate) mod atx_heading;
pub(crate) mod indented_code_block;
pub(crate) mod stx_heading;
pub(crate) mod thematic_break;


pub enum SubState {
    ATXHeading(ATXHeadingState),
    STXHeading(STXHeadingState),
    ThematicBreak(ThematicBreakState),
    IndentedCodeBlock(IndentedCodeBlockState),
    FencedCodeBlock(FencedCodeBlockState),
    IntoSuper(State),
}

impl Transition for SubState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            SubState::ATXHeading(state) => state.transition(character),
            SubState::STXHeading(state) => state.transition(character),
            SubState::ThematicBreak(state) => state.transition(character),
            SubState::IndentedCodeBlock(state) => state.transition(character),
            SubState::FencedCodeBlock(state) => state.transition(character),
            SubState::IntoSuper(_) => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            SubState::ATXHeading(state) => state.end_line(line_ending),
            SubState::STXHeading(state) => state.end_line(line_ending),
            SubState::ThematicBreak(state) => state.end_line(line_ending),
            SubState::IndentedCodeBlock(state) => state.end_line(line_ending),
            SubState::FencedCodeBlock(state) => state.end_line(line_ending),
            SubState::IntoSuper(_) => NonDeterministicTransitionEffect::dismiss(),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            SubState::ATXHeading(state) => state.end(),
            SubState::STXHeading(state) => state.end(),
            SubState::ThematicBreak(state) => state.end(),
            SubState::IndentedCodeBlock(state) => state.end(),
            SubState::FencedCodeBlock(state) => state.end(),
            SubState::IntoSuper(_) => None,
        }
    }
}

pub struct SubStates(Vec<SubState>);

impl SubStates {
    pub fn push(&mut self, value: impl Into<SubState>) {
        self.0.push(value.into());
    }

    pub fn states(self) -> Vec<SubState> {
        self.0
    }

    pub fn last(&self) -> Option<&SubState> {
        self.0.last()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for SubStates {
    fn default() -> Self {
        Self(vec![])
    }
}