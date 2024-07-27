use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{State, Transition};
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::potential::PotentialState;
use crate::parser::state::thematic_break::ThematicBreakState;
use crate::unicode;

#[derive(Clone)]
pub struct DefaultState {
    leading_spaces: usize,
}

impl Transition for DefaultState {
    fn transition(self, character: Character) -> Action {
        if let Character::Unescaped(unicode::SPACE) = character {
            return Action::Pass(
                State::Default(
                    DefaultState {
                        leading_spaces: self.leading_spaces + 1,
                    }
                )
            );
        }

        let mut states = Vec::new();

        if self.leading_spaces < 4 {
            if let Ok(state ) = ThematicBreakState::try_from(character) {
                states.push(
                    State::ThematicBreak(state)
                );
            }

            if let Ok(state) = ATXHeadingState::try_from(character) {
                states.push(
                    State::ATXHeading(state)
                );
            }
        }


        Action::Pass(
            State::Potential(PotentialState::new(states))
        )
    }

    fn end(self) -> Action {
        Action::Pass(
            State::Default(DefaultState::default())
        )
    }
}

impl Default for DefaultState {
    fn default() -> Self {
        DefaultState {
            leading_spaces: 0
        }
    }
}