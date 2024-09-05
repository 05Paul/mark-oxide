use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::stx_heading::content::ContentState;
use crate::parser::state::sub_state::stx_heading::newline::NewlineState;
use crate::parser::state::sub_state::stx_heading::{STXHeadingSubState, UnderlineCharacter};
use crate::parser::state::sub_state::stx_heading::trailing::TrailingState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

/// # Setext Heading: Underline
/// Setext underline
///
/// ## Transition
/// * [TrailingState] on whitespace character
/// * [ContentState] on non-whitespace character
///
/// ## Completion
/// * LineEnding
/// * DocumentEnding
pub struct UnderlineState {
    pub(crate) content: String,
    pub(crate) temp: String,
    pub(crate) underline_character: UnderlineCharacter,
    pub(crate) underline_character_count: usize,
}

impl Transition for UnderlineState {
    type Effect = NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        if let Ok(character) = UnderlineCharacter::try_from(&character) {
            if character == self.underline_character {
                self.underline_character_count += 1;
                self.temp.push(character.character());
                return NonDeterministicTransitionEffect::pass(self);
            }
        }

        match character {
            character @ Character::Unescaped(unicode::SPACE | unicode::TAB) => {
                self.temp.push_str(&*character.to_string());
                NonDeterministicTransitionEffect::transition_into::<TrailingState>(self)
            },
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

impl From<(NewlineState, UnderlineCharacter)> for UnderlineState {
    fn from((state, underline): (NewlineState, UnderlineCharacter)) -> Self {
        Self {
            content: state.content,
            temp: state.temp,
            underline_character: underline,
            underline_character_count: 0,
        }
    }
}

impl From<UnderlineState> for STXHeadingSubState {
    fn from(value: UnderlineState) -> Self {
        STXHeadingSubState::Underline(value)
    }
}