use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::{ATXHeadingSubState, HASHTAG};
use crate::parser::state::sub_state::atx_heading::content::ContentState;
use crate::parser::state::sub_state::atx_heading::leading::LeadingWhitespaceState;
use crate::parser::state::sub_state::atx_heading::trailing::PotentiallyTrailingState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # ATXHeading: Potential closing sequence
/// Potential closing sequence consisting of [HASHTAG]
///
/// ## Transition
/// * [PotentiallyTrailingState] on whitespace character
/// * [ContentState] on non-whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct PotentiallyClosingState {
    pub(crate) level: usize,
    pub(crate) content: String,
    pub(crate) temp: String,
}

impl Transition for PotentiallyClosingState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(HASHTAG) => {
                self.temp.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::pass(self)
            },
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.temp.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::transition_into::<PotentiallyTrailingState>(self)
            }
            character => {
                self.content.push_str(&*self.temp);
                self.content.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
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

impl From<(LeadingWhitespaceState, Character)> for PotentiallyClosingState {
    fn from((state, character): (LeadingWhitespaceState, Character)) -> Self {
        Self {
            level: state.level,
            content: "".into(),
            temp: character.to_string(),
        }
    }
}

impl From<PotentiallyTrailingState> for PotentiallyClosingState {
    fn from(value: PotentiallyTrailingState) -> Self {
        Self {
            level: value.level,
            content: value.content,
            temp: value.temp,
        }
    }
}

impl From<PotentiallyClosingState> for ATXHeadingSubState {
    fn from(value: PotentiallyClosingState) -> Self {
        ATXHeadingSubState::PotentiallyClosing(value)
    }
}