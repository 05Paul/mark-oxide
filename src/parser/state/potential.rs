use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{LineEnding, State, Transition};

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
        let mut completion = Action::Dismiss;

        for state in self.states {
            match state.transition(character) {
                Action::Complete(block) => {
                    completion = Action::Complete(block);
                    break;
                }
                Action::Pass(state) => states.push(state),
                Action::Dismiss => {}
                Action::Bi { first, second } =>
                    match (first.as_ref().clone(), second.as_ref().clone()) {
                        (Action::Complete(block), Action::Pass(state)) => {
                            completion = Action::Complete(block);
                            states.push(state);
                            break;
                        },
                        _ => unreachable!(),
                    },
            }
        }

        let pass = if states.is_empty() {
            Action::Dismiss
        } else {
            Action::Pass(
                State::Potential(PotentialState::new(states)))
        };

        completion.merge(pass)
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

    fn end_line(self, line_ending: LineEnding) -> Action {
        let mut states = Vec::new();
        for state in self.states {
            match state.end_line(line_ending) {
                Action::Complete(block) => {
                    return Action::Complete(block);
                }
                Action::Pass(state) => states.push(state),
                _ => {}
            }
        }

        if states.is_empty() {
            Action::Dismiss
        } else {
            Action::Pass(
                State::Potential(
                    PotentialState::new(states)
                )
            )
        }
    }
}