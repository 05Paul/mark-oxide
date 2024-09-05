use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::state::sub_state::fenced_code_block::opening::OpeningState;
use crate::parser::state::sub_state::fenced_code_block::opening_trail::OpeningTrailingState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Fenced Code Block: info string
/// Code block info string
///
/// ## Transition
/// * [OpeningTrailingState] on whitespace character
/// * [NewlineState] on LineEnding
///
/// ## Dismissal
/// * DocumentEnding
/// * on [BACKTICK] when FenceCharacter is [BACKTICK]
pub struct InfoStringState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
    pub(crate) info_string: String,
}

impl Transition for InfoStringState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                if self.info_string.is_empty() {
                    NonDeterministicTransitionEffect::pass(self)
                } else {
                    NonDeterministicTransitionEffect::transition_into::<OpeningTrailingState>(self)
                }
            }
            Character::Unescaped(fenced_code_block::BACKTICK) => {
                match self.character {
                    FenceCharacter::Backtick => {
                        NonDeterministicTransitionEffect::dismiss()
                    }
                    FenceCharacter::Tilde => {
                        self.info_string.push_str(&*character.to_string());
                        NonDeterministicTransitionEffect::pass(self)
                    }
                }
            }
            character => {
                self.info_string.push_str(&*character.to_string());
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

impl From<(OpeningState, Character)> for InfoStringState {
    fn from((state, character): (OpeningState, Character)) -> Self {
        Self {
            character: state.character,
            character_count: state.character_count,
            indentation: state.indentation,
            info_string: character.to_string().trim().to_string(),
        }
    }
}

impl From<InfoStringState> for FencedCodeBlockSubState {
    fn from(value: InfoStringState) -> Self {
        FencedCodeBlockSubState::Info(value)
    }
}