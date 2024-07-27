use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::thematic_break::ThematicBreakState;
use crate::parser::state::{State, SubTransition, Transition};
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::potential::PotentialState;

#[derive(Clone)]
pub struct DefaultState;

impl Transition for DefaultState {
    fn transition(self, character: Character) -> Action {
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

        Action::Pass(
            State::Potential(PotentialState::new(states))
        )


    }

    fn end(self) -> Action {
        Action::Pass(
            State::Default(DefaultState)
        )
    }
}