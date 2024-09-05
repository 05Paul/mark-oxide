use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::DeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::default::DefaultState;
use crate::parser::state::potential::PotentialState;
use crate::parser::transition::{Transition, TransitionEffect};

mod default;
mod potential;
mod sub_state;
mod handler;

type HandlerTransitionEffect = DeterministicTransitionEffect<(), Option<Block>>;

pub struct StateHandler(Option<State>);

impl Transition for &mut StateHandler {
    type Effect = HandlerTransitionEffect;

    fn transition(self, character: Character) -> Self::Effect {
        let transition = match self.0.take() {
            None => return HandlerTransitionEffect::dismiss(),
            Some(state) => state.transition(character),
        };
        let (state, block) = transition.content();

        self.0 = Some(state);

        HandlerTransitionEffect::complete(block)
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        let transition = match self.0.take() {
            None => return HandlerTransitionEffect::dismiss(),
            Some(state) => state.end_line(line_ending),
        };
        let (state, block) = transition.content();

        self.0 = Some(state);

        HandlerTransitionEffect::complete(block)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        self.0.take()?.end()
    }
}

impl Default for StateHandler {
    fn default() -> Self {
        Self(Some(Default::default()))
    }
}

pub enum State {
    Default(DefaultState),
    Potential(PotentialState),
    /*
    ATXHeading(ATXHeadingState),
    SetextHeading(SetextHeadingState),
    ThematicBreak(ThematicBreakState),
    IndentedCodeBlock(IndentedCodeBlockState),
    FencedCodeBlock(FencedCodeBlockState),

     */
}

impl State {
    pub fn from_leading_space_count(leading_spaces: usize) -> Self {
        State::Default(
            DefaultState::from(leading_spaces)
        )
    }
}

impl Transition for State {
    type Effect = DeterministicTransitionEffect<State, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            State::Default(state) => state.transition(character),
            State::Potential(state) => state.transition(character),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            State::Default(state) => state.end_line(line_ending),
            State::Potential(state) => state.end_line(line_ending),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            State::Default(state) => state.end(),
            State::Potential(state) => state.end()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Default(DefaultState::default())
    }
}