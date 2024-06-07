use crate::state::{Action, Character, State, Transition};
use crate::state::default::DefaultState;

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
    fn transition(self, character: Character) -> (State, Action) {
        let mut states = Vec::new();

        for state in self.states {
            let (state, action) = state.transition(character);
            match action {
                Action::Complete(_) => {
                    return (
                        State::Default(DefaultState),
                        action,
                    );
                }
                Action::Pass => states.push(state),
                Action::Dismiss => {}
            }
        }

        (
            State::Potential(PotentialState::new(states)),
            Action::Pass
        )
    }

    fn end(self) -> (State, Action) {
        for state in self.states {
            let (_, action) = state.end();
            match action {
                Action::Complete(_) => {
                    return (
                        State::Default(DefaultState),
                        action
                    );
                }
                _ => {}
            }
        }

        (
            State::Default(DefaultState),
            Action::Dismiss,
        )
    }
}