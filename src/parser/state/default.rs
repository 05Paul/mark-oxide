use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{LineEnding, State, Transition};
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::indented_code_block::IndentedCodeBlockState;
use crate::parser::state::potential::PotentialState;
use crate::parser::state::thematic_break::ThematicBreakState;

#[derive(Clone)]
pub struct DefaultState {
    leading_spaces: usize,
}

impl Transition for DefaultState {
    fn transition(self, character: Character) -> Action {
        let mut states = Vec::new();

        if character.is_blank() {
            if self.leading_spaces + character.space_count() >= 4 {
                states.push(
                    State::IndentedCodeBlock(
                        IndentedCodeBlockState::new(self.leading_spaces + character.space_count() - 4)
                    )
                )
            } else {
                return Action::Pass(
                    State::Default(
                        DefaultState {
                            leading_spaces: self.leading_spaces + character.space_count(),
                        }
                    )
                );
            }
        }

        if self.leading_spaces < 4 {
            if let Ok(state) = ThematicBreakState::try_from(character) {
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

    fn end_line(self, _: LineEnding) -> Action {
        self.end()
    }
}

impl Default for DefaultState {
    fn default() -> Self {
        DefaultState {
            leading_spaces: 0
        }
    }
}