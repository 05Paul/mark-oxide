use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::{ATXHeadingSubState, HASHTAG, LeadingWhitespaceState};
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # ATX Heading: Opening sequence
/// Opening sequence of up to 6 [HASHTAG] determining the level
///
/// ## Transition
/// * [LeadingWhitespace] on whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
///
/// ## Dismissal
/// * Exceeding 6 [HASHTAG]
pub struct OpeningSequenceState {
    pub(crate) level: usize,
}

impl Transition for OpeningSequenceState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (character, self.level) {
            (Character::Unescaped(HASHTAG), 0..=5) => {
                self.level += 1;
                NonDeterministicTransitionEffect::pass(self)
            },
            (Character::Unescaped(unicode::SPACE | unicode::TAB), _) => {
                NonDeterministicTransitionEffect::transition_into::<LeadingWhitespaceState>(self)
            },
            _ => NonDeterministicTransitionEffect::dismiss(),
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

impl From<OpeningSequenceState> for ATXHeadingSubState {
    fn from(value: OpeningSequenceState) -> Self {
        ATXHeadingSubState::Opening(value)
    }
}