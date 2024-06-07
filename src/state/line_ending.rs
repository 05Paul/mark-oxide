use crate::state::{Action, Character, State, SubTransition, Transition};
use crate::state::default::DefaultState;
use crate::unicode;

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
    fn transition(self, character: Character) -> (State, Action) {
        match (self.character, character.character()) {
            (unicode::CARRIAGE_RETURN, unicode::LINE_FEED) => self.previous_state.end(),
            _ => {
                let (_, action) = self.previous_state.end();
                let (state, _) = DefaultState.transition(character);
                (state, action)
            }
        }
    }

    fn end(self) -> (State, Action) {
        self.previous_state.end()
    }
}

impl SubTransition for LineEndingState {
    fn is_start(value: Character) -> bool {
        value.character() == unicode::LINE_FEED || value.character() == unicode::CARRIAGE_RETURN
    }
}