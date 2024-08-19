use std::cell::OnceCell;

use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::DeterministicTransitionEffect;
use crate::parser::state::{LineEnding, State};
use crate::parser::state::sub_state::{SubState, SubStates};
use crate::parser::transition::{Transition, TransitionEffect};

#[derive(Clone)]
pub struct PotentialState {
    states: SubStates,
}

impl PotentialState {
    fn handle_transition(self, mut result: OnceCell<Block>) -> DeterministicTransitionEffect<State, Option<Block>> {
        if let Some(block) = result.take() {
            let into = match self.states.last() {
                Some(SubState::IntoSuper(_)) => match self.states.states().pop() {
                    Some(SubState::IntoSuper(state)) => Some(state),
                    _ => None,
                },
                _ => None,
            };

            match into {
                Some(state) => DeterministicTransitionEffect::new(
                    state,
                    Some(block)
                ),
                None => DeterministicTransitionEffect::complete(block),
            }

        } else if self.states.is_empty() {
            DeterministicTransitionEffect::dismiss()
        } else {
            DeterministicTransitionEffect::pass(self)
        }
    }
}

impl Transition for PotentialState {
    type Effect = DeterministicTransitionEffect<State, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        let mut retained_states = SubStates::default();
        let completion = OnceCell::new();

        for state in self.states.states() {
            let (state, block) = state.transition(character.clone())
                .content();
            if let Some(state) = state {
                retained_states.push(state);
            }

            if let Some(block) = block {
                let  _ = completion.set(block);
            }

        }

        self.states = retained_states;

        self.handle_transition(completion)
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        let mut retained_states = SubStates::default();
        let completion = OnceCell::new();

        for state in self.states.states() {
            let (state, block) = state.end_line(line_ending)
                .content();
            if let Some(state) = state {
                retained_states.push(state);
            }

            if let Some(block) = block {
                let  _ = completion.set(block);
            }

        }

        self.states = retained_states;

        self.handle_transition(completion)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        let mut completion = OnceCell::new();
        for state in self.states.states() {
            let result = state.end();
            match result {
                Some(block) => {
                    let _ = completion.set(block);
                }
                _ => {}
            }
        }

        completion.take()
    }
}

impl From<SubStates> for PotentialState {
    fn from(value: SubStates) -> Self {
        Self {
            states: value,
        }
    }
}

impl From<PotentialState> for State {
    fn from(value: PotentialState) -> Self {
        State::Potential(value)
    }
}