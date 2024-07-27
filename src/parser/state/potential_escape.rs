use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{State, SubTransition, Transition};
use crate::parser::state::default::DefaultState;

#[derive(Clone)]
pub struct PotentialEscapeState {
    previous_state: Box<State>,
}

impl PotentialEscapeState {
    pub fn new(state: State) -> Self {
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
    fn transition(self, character: Character) -> Action {
        if PotentialEscapeState::escapable(&character.character()) {
            self.previous_state.transition(Character::Escaped(character.character()))
        } else {
            let action = self.previous_state.transition(Character::Unescaped('\\'));
            match action {
                Action::Pass(state) => state.transition(
                    Character::Unescaped(character.character())
                ),
                Action::Dismiss => State::Default(DefaultState).transition(
                    Character::Unescaped(character.character())
                ),
                Action::Complete(_) => unreachable!("Should not be reached"),
                Action::Bi { .. } => unreachable!(),
            }
        }
    }

    fn end(self) -> Action {
        let action = self.previous_state.transition(Character::Unescaped('\\'));

        match action {
            Action::Pass(State::PotentialEscape(_)) => Action::Pass(State::Default(DefaultState)),
            Action::Pass(state) => state.end(),
            _ => action,
        }
    }
}

impl SubTransition for PotentialEscapeState {
    fn is_start(value: Character) -> bool {
        match value {
            Character::PotentiallyEscaped('\\') => true,
            _ => false,
        }
    }
}