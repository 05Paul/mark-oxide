use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::ATXHeadingSubState;
use crate::parser::state::sub_state::atx_heading::closing::PotentiallyClosingState;
use crate::parser::state::sub_state::atx_heading::leading::LeadingWhitespaceState;
use crate::parser::state::sub_state::atx_heading::trailing::PotentiallyTrailingState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # ATX Heading: Content
/// Content of the heading
///
/// ## Transition
/// * [PotentiallyTrailingState] on whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct ContentState {
    pub(crate) level: usize,
    pub(crate) content: String,
}

impl Transition for ContentState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                NonDeterministicTransitionEffect::transition_into::<PotentiallyTrailingState>((
                    self,
                    character,
                ))
            }
            character => {
                self.content.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::pass(self)
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

impl From<(LeadingWhitespaceState, Character)> for ContentState {
    fn from((state, character): (LeadingWhitespaceState, Character)) -> Self {
        Self {
            level: state.level,
            content: character.to_string(),
        }
    }
}

impl From<PotentiallyTrailingState> for ContentState {
    fn from(value: PotentiallyTrailingState) -> Self {
        Self {
            level: value.level,
            content: value.content,
        }
    }
}

impl From<PotentiallyClosingState> for ContentState {
    fn from(value: PotentiallyClosingState) -> Self {
        Self {
            level: value.level,
            content: value.content,
        }
    }
}

impl From<ContentState> for ATXHeadingSubState {
    fn from(value: ContentState) -> Self {
        ATXHeadingSubState::Content(value)
    }
}
