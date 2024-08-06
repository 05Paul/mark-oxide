use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{LineEnding, State, Transition};
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::fenced_code_block::FencedCodeBlockState;
use crate::parser::state::indented_code_block::IndentedCodeBlockState;
use crate::parser::state::potential::PotentialState;
use crate::parser::state::setext_heading::SetextHeadingState;
use crate::parser::state::thematic_break::ThematicBreakState;

#[derive(Clone)]
pub struct DefaultState {
    leading_spaces: usize,
}

impl DefaultState {
    pub fn new(leading_spaces: usize) -> Self {
        Self {
            leading_spaces,
        }
    }
}

impl Transition for DefaultState {
    fn transition(self, character: Character) -> Action {
        let leading_spaces = self.leading_spaces + character.space_count();
        let mut states = Vec::new();

        if character.is_blank() {
            if leading_spaces >= 4 {
                states.push(
                    State::IndentedCodeBlock(
                        IndentedCodeBlockState::new(leading_spaces - 4)
                    )
                )
            } else {
                return Action::Pass(
                    State::Default(
                        DefaultState {
                            leading_spaces,
                        }
                    )
                );
            }
        }

        if leading_spaces < 4 {
            states.push(
                State::SetextHeading(
                    SetextHeadingState::new(character)
                )
            );

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

            if let Ok(state) = FencedCodeBlockState::new(leading_spaces, character) {
                states.push(
                    State::FencedCodeBlock(state)
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