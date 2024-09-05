use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::atx_heading::closing::PotentiallyClosingState;
use crate::parser::state::sub_state::atx_heading::content::ContentState;
use crate::parser::state::sub_state::atx_heading::{ATXHeadingSubState, HASHTAG};
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # ATX Heading: Trailing whitespaces
/// Potentially trailing whitespace characters
///
/// ## Transition
/// * [PotentiallyClosingState] on [HASHTAG]
/// * [ContentState] on non-whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct PotentiallyTrailingState {
    pub(crate) level: usize,
    pub(crate) content: String,
    pub(crate) temp: String,
}

impl Transition for PotentiallyTrailingState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.temp.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::pass(self)
            }
            character @ Character::Unescaped(HASHTAG) => {
                if let Some(index) = self.temp.rfind(HASHTAG) {
                    let (prev, trail) = self.temp.split_at(index);
                    self.content.push_str(prev);
                    self.temp = String::from(trail);
                };

                self.temp.push_str(&*character.to_string());

                NonDeterministicTransitionEffect::transition_into::<PotentiallyClosingState>(self)
            },
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

impl From<(ContentState, Character)> for PotentiallyTrailingState {
    fn from((state, character): (ContentState, Character)) -> Self {
        Self {
            level: state.level,
            content: state.content,
            temp: character.to_string(),
        }
    }
}

impl From<PotentiallyClosingState> for PotentiallyTrailingState {
    fn from(value: PotentiallyClosingState) -> Self {
        Self {
            level: value.level,
            content: value.content,
            temp: value.temp,
        }
    }
}

impl From<PotentiallyTrailingState> for ATXHeadingSubState {
    fn from(value: PotentiallyTrailingState) -> Self {
        ATXHeadingSubState::PotentiallyTrailing(value)
    }
}
