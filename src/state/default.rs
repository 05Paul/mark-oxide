use crate::state::{Action, Character, State, SubTransition, Transition};
use crate::state::atx_heading::ATXHeadingState;
use crate::state::potential::PotentialState;
use crate::state::thematic_break::ThematicBreakState;

pub struct DefaultState;

impl Transition for DefaultState {
    fn transition(self, character: Character) -> (State, Action) {
        let mut states = Vec::new();

        if ThematicBreakState::is_start(character) {
            states.push(
                State::ThematicBreak(
                    ThematicBreakState::new(character)
                )
            );
        }

        if ATXHeadingState::is_start(character) {
            states.push(
                State::ATXHeading(
                    ATXHeadingState::new(character)
                )
            );
        }

        (State::Potential(PotentialState::new(states)), Action::Pass)

    }

    fn end(self) -> (State, Action) {
        (State::Default(DefaultState), Action::Pass)
    }
}