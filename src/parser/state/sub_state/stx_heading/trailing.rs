use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::stx_heading::content::ContentState;
use crate::parser::state::sub_state::stx_heading::underline::UnderlineState;
use crate::parser::state::sub_state::stx_heading::{STXHeadingSubState, UnderlineCharacter};
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Setext Heading: Trail
/// Setext underline trail
///
/// ## Transition
/// * [ContentState] on non-whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct TrailingState {
    pub(crate) content: String,
    pub(crate) underline_character: UnderlineCharacter,
    pub(crate) temp: String,
}

impl Transition for TrailingState {
    type Effect = NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.temp.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::pass(self)
            }
            character => {
                self.content.push_str(&*self.temp);
                self.content.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
            }
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        STXHeadingSubState::complete(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        STXHeadingSubState::complete(self)
            .end()
    }
}

impl From<UnderlineState> for TrailingState {
    fn from(value: UnderlineState) -> Self {
        Self {
            content: value.content,
            underline_character: value.underline_character,
            temp: value.temp,
        }
    }
}

impl From<TrailingState> for STXHeadingSubState {
    fn from(value: TrailingState) -> Self {
        STXHeadingSubState::Trailing(value)
    }
}