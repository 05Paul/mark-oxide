use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{State, SubTransition, Transition};
use crate::parser::state::default::DefaultState;
use crate::unicode;

#[derive(Clone)]
pub struct LineEndingState {
    character: char,
    previous_state: Box<State>,
}

impl LineEndingState {
    pub fn new(character: char, state: State) -> Self {
        Self {
            character,
            previous_state: Box::new(state),
        }
    }
}

impl Transition for LineEndingState {
    fn transition(self, character: Character) -> Action {
        match (self.character, character.character()) {
            (unicode::CARRIAGE_RETURN, unicode::LINE_FEED) => self.previous_state.end(),
            _ => {
                let x = self.previous_state.end();
                match x {
                    Action::Pass(state) => state.transition(character),
                    Action::Complete(block) => Action::Complete(block)
                        .merge(
                            State::default()
                                .transition(character)
                        ),
                    Action::Dismiss => State::default().transition(character),
                    Action::Bi { .. } => unreachable!(),
                }
            }
        }
    }

    fn end(self) -> Action {
        self.previous_state.end()
    }
}

impl SubTransition for LineEndingState {
    fn is_start(value: Character) -> bool {
        value.character() == unicode::LINE_FEED || value.character() == unicode::CARRIAGE_RETURN
    }
}