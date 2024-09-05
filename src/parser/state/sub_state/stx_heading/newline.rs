use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::stx_heading::{STXHeadingSubState, UnderlineCharacter};
use crate::parser::state::sub_state::stx_heading::content::ContentState;
use crate::parser::state::sub_state::stx_heading::underline::UnderlineState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Setext Heading: Newline
/// Setext newline
///
/// ## Transition**
/// * [UnderlineState] on [UnderlineCharacter]
/// * [ContentState] on non-space character
///
/// ## Dismissal
/// * LineEnding
/// * DocumentEnding
pub struct NewlineState {
    pub(crate) content: String,
    pub(crate) temp: String,
    pub(crate) leading_spaces: usize,
}

impl Transition for NewlineState {
    type Effect = NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        if let Ok(underline) = UnderlineCharacter::try_from(&character) {
            NonDeterministicTransitionEffect::transition_into::<UnderlineState>((self, underline))
        } else if let Character::Unescaped(unicode::SPACE) = &character {
            self.leading_spaces += character.space_count();
            self.temp.push(unicode::SPACE);

            if self.leading_spaces >= 4 {
                self.content.push_str(&*self.temp);
                NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
            } else {
                NonDeterministicTransitionEffect::pass(self)
            }
        } else {
            self.content.push_str(&*self.temp);
            self.content.push_str(&*character.to_string());
            NonDeterministicTransitionEffect::transition_into::<ContentState>(self)
        }
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        NonDeterministicTransitionEffect::dismiss()
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        None
    }
}

impl From<(ContentState, LineEnding)> for NewlineState {
    fn from((state, line_ending): (ContentState, LineEnding)) -> Self {
        Self {
            content: state.content,
            temp: line_ending.to_string(),
            leading_spaces: 0,
        }
    }
}

impl From<NewlineState> for STXHeadingSubState {
    fn from(value: NewlineState) -> Self {
        STXHeadingSubState::Newline(value)
    }
}