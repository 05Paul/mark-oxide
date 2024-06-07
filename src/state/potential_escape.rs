use crate::state::{Action, Character, State, SubTransition, Transition};
use crate::state::default::DefaultState;

pub struct PotentialEscapeState {
    previous_state: Box<State>,
}

impl PotentialEscapeState {
    fn new(state: State) -> Self {
        Self {
            previous_state: Box::new(state),
        }
    }

    fn escapable(character: &char) -> bool {
        [
            '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+',
            ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@',
            '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
        ].contains(character)
    }
}

impl Transition for PotentialEscapeState {
    fn transition(self, character: Character) -> (State, Action) {
        if PotentialEscapeState::escapable(&character.character()) {
            self.previous_state.transition(Character::Escaped(character.character()))
        } else {
            let (state, action) = self.previous_state.transition(Character::Unescaped('\\'));
            match action {
                Action::Pass | Action::Dismiss => state.transition(Character::Unescaped(character.character())),
                Action::Complete(_) => {
                    let (state, _) = state.transition(Character::Unescaped(character.character()));
                    (state, action)
                }
            }
        }
    }

    fn end(self) -> (State, Action) {
        let (state, action) = self.previous_state.transition(Character::Unescaped('\\'));
        match action {
            Action::Pass | Action::Dismiss => state.end(),
            Action::Complete(_) => (
                State::Default(DefaultState),
                action,
            ),
        }
    }
}

impl SubTransition for PotentialEscapeState {
    fn is_start(value: Character) -> bool {
        value.character() == '\\'
    }
}