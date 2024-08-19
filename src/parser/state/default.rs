use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::DeterministicTransitionEffect;
use crate::parser::state::{LineEnding, State};
use crate::parser::state::potential::PotentialState;
use crate::parser::state::sub_state::atx_heading::ATXHeadingState;
use crate::parser::state::sub_state::fenced_code_block::FencedCodeBlockState;
use crate::parser::state::sub_state::indented_code_block::IndentedCodeBlockState;
use crate::parser::state::sub_state::stx_heading::STXHeadingState;
use crate::parser::state::sub_state::SubStates;
use crate::parser::state::sub_state::thematic_break::ThematicBreakState;
use crate::parser::transition::{Transition, TransitionEffect};

#[derive(Clone)]
pub struct DefaultState {
    leading_spaces: usize,
}

impl Transition for DefaultState {
    type Effect = DeterministicTransitionEffect<State, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        let mut states = SubStates::default();

        self.leading_spaces += character.space_count();

        if character.is_blank() {
            if self.leading_spaces >= 4 {
                states.push(
                    IndentedCodeBlockState::new(self.leading_spaces - 4)
                )
            } else {
                return DeterministicTransitionEffect::pass(self);
            }
        }

        if self.leading_spaces < 4 {
            states.push(
                STXHeadingState::new(character.clone())
            );

            if let Ok(state) = ThematicBreakState::try_from(character.clone()) {
                states.push(state);
            }

            if let Ok(state) = ATXHeadingState::try_from(character.clone()) {
                states.push(state);
            }

            if let Ok(state) = FencedCodeBlockState::new(self.leading_spaces, character.clone()) {
                states.push(state);
            }
        }


        DeterministicTransitionEffect::transition_into::<PotentialState>(states)
    }

    fn end_line(mut self, _: LineEnding) -> Self::Effect {
        self.leading_spaces = 0;
        DeterministicTransitionEffect::pass(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        None
    }
}

impl Default for DefaultState {
    fn default() -> Self {
        DefaultState {
            leading_spaces: 0
        }
    }
}

impl From<usize> for DefaultState {
    fn from(value: usize) -> Self {
        Self {
            leading_spaces: value,
        }
    }
}

impl From<DefaultState> for State {
    fn from(value: DefaultState) -> Self {
        State::Default(value)
    }
}