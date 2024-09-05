use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::{ATXHeadingSubState, ContentState, HASHTAG};
use crate::parser::state::sub_state::atx_heading::closing::PotentiallyClosingState;
use crate::parser::state::sub_state::atx_heading::opening::OpeningSequenceState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # ATX Heading: Leading whitespaces
/// Whitespace characters preceding the content of the heading
///
/// ## Transition
/// * [PotentiallyClosingState] on [HASHTAG]
/// * [ContentState] on non-whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct LeadingWhitespaceState {
    pub(crate) level: usize,
}

impl Transition for LeadingWhitespaceState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match character {
            Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                NonDeterministicTransitionEffect::pass(self)
            }
            character @ Character::Unescaped(HASHTAG) => {
                NonDeterministicTransitionEffect::transition_into::<PotentiallyClosingState>((
                    self,
                    character,
                ))
            }
            character => {
                NonDeterministicTransitionEffect::transition_into::<ContentState>((
                    self,
                    character,
                ))
            }
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        ATXHeadingSubState::complete(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        ATXHeadingSubState::complete(self)
            .end()
    }
}

impl From<LeadingWhitespaceState> for ATXHeadingSubState {
    fn from(value: LeadingWhitespaceState) -> Self {
        ATXHeadingSubState::LeadingWhitespace(value)
    }
}

impl From<OpeningSequenceState> for LeadingWhitespaceState {
    fn from(value: OpeningSequenceState) -> Self {
        Self {
            level: value.level,
        }
    }
}