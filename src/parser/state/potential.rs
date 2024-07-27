use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{State, Transition};

#[derive(Clone)]
pub struct PotentialState {
    states: Vec<State>,
}

impl PotentialState {
    pub fn new(states: Vec<State>) -> Self {
        Self {
            states
        }
    }
}

impl Transition for PotentialState {
    fn transition(self, character: Character) -> Action {
        let mut states = Vec::new();

        for state in self.states {
            match state.transition(character) {
                Action::Complete(block) => {
                    return Action::Complete(block);
                }
                Action::Pass(state) => states.push(state),
                Action::Dismiss => {},
                Action::Bi { .. } => unreachable!(),
            }
        }

        Action::Pass(
            State::Potential(PotentialState::new(states))
        )
    }

    fn end(self) -> Action {
        for state in self.states {
            match state.end() {
                Action::Complete(block) => {
                    return Action::Complete(
                        block
                    );
                }
                _ => {}
            }
        }

        Action::Dismiss
    }
}