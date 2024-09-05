use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::info::InfoStringState;
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::transition::{Transition, TransitionEffect};

/// # Fenced Code Block: Opening fence trail
/// Opening fence trail
///
/// ## Transition
/// * [NewlineState] on LineEnding
///
/// ## Dismissal
/// * DocumentEnding
/// * on [BACKTICK] when FenceCharacter is [BACKTICK]
pub struct OpeningTrailingState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
    pub(crate) info_string: String,
}

impl Transition for OpeningTrailingState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match character {
            Character::Unescaped(fenced_code_block::BACKTICK) => {
                match self.character {
                    FenceCharacter::Backtick => {
                        NonDeterministicTransitionEffect::dismiss()
                    }
                    FenceCharacter::Tilde => {
                        NonDeterministicTransitionEffect::pass(self)
                    }
                }
            }
            _ => {
                NonDeterministicTransitionEffect::pass(self)
            }
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        NonDeterministicTransitionEffect::transition_into::<NewlineState>(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        None
    }
}

impl From<InfoStringState> for OpeningTrailingState {
    fn from(value: InfoStringState) -> Self {
        Self {
            character: value.character,
            character_count: value.character_count,
            indentation: value.indentation,
            info_string: value.info_string,
        }
    }
}

impl From<OpeningTrailingState> for FencedCodeBlockSubState {
    fn from(value: OpeningTrailingState) -> Self {
        FencedCodeBlockSubState::OpeningTrail(value)
    }
}