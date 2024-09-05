use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::fenced_code_block;
use crate::parser::state::sub_state::fenced_code_block::{FenceCharacter, FencedCodeBlockSubState};
use crate::parser::state::sub_state::fenced_code_block::info::InfoStringState;
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Fenced Code Block: Opening Fence
/// Opening sequence of up to N [FenceCharacter]s
///
/// ## Transition
/// * [InfoStringState] on non-whitespace character
/// * [NewlineState] on LineEnding
///
/// ## Dismissal
/// * DocumentEnding
/// * on [BACKTICK] when [FenceCharacter] is [BACKTICK]
pub struct OpeningState {
    pub(crate) character: FenceCharacter,
    pub(crate) character_count: usize,
    pub(crate) indentation: usize,
}

impl OpeningState {
    pub fn new(indentation: usize, character: Character) -> Result<Self, Error> {
        Ok(
            Self {
                character: FenceCharacter::try_from(&character)?,
                character_count: 1,
                indentation,
            }
        )
    }
}

impl Transition for OpeningState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                if self.character_count >= 3 {
                    NonDeterministicTransitionEffect::transition_into::<InfoStringState>((
                        self,
                        character
                    ))
                } else {
                    NonDeterministicTransitionEffect::dismiss()
                }
            }
            character @ Character::Unescaped(fenced_code_block::BACKTICK) => {
                match self.character {
                    FenceCharacter::Backtick => {
                        self.character_count += 1;
                        NonDeterministicTransitionEffect::pass(self)
                    }
                    FenceCharacter::Tilde => {
                        NonDeterministicTransitionEffect::transition_into::<InfoStringState>((
                            self,
                            character,
                        ))
                    }
                }
            }
            character @ Character::Unescaped(fenced_code_block::TILDE) => {
                match self.character {
                    FenceCharacter::Tilde => {
                        self.character_count += 1;
                        NonDeterministicTransitionEffect::pass(self)
                    }
                    FenceCharacter::Backtick => {
                        NonDeterministicTransitionEffect::transition_into::<InfoStringState>((
                            self,
                            character,
                        ))
                    }
                }
            }
            character => {
                NonDeterministicTransitionEffect::transition_into::<InfoStringState>((
                    self,
                    character,
                ))
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

impl From<OpeningState> for FencedCodeBlockSubState {
    fn from(value: OpeningState) -> Self {
        FencedCodeBlockSubState::Opening(value)
    }
}