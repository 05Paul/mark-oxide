use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::sub_state::stx_heading::newline::NewlineState;
use crate::parser::state::sub_state::stx_heading::STXHeadingSubState;
use crate::parser::state::sub_state::stx_heading::trailing::TrailingState;
use crate::parser::state::sub_state::stx_heading::underline::UnderlineState;
use crate::parser::transition::{Transition, TransitionEffect};

/// # Setext Heading: Content
/// Setext heading content
///
/// ## Transition
/// * [NewlineState] on LineEnding
///
/// ## Dismissal
/// * DocumentEnding
pub struct ContentState {
    pub(crate) content: String,
}

impl ContentState {
    pub fn new(character: Character) -> Self {
        Self {
            content: character.to_string(),
        }
    }
}

impl Transition for ContentState {
    type Effect = NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        self.content.push_str(&*character.to_string());
        NonDeterministicTransitionEffect::pass(self)
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        NonDeterministicTransitionEffect::transition_into::<NewlineState>((self, line_ending))
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        None
    }
}

impl From<NewlineState> for ContentState {
    fn from(value: NewlineState) -> Self {
        Self {
            content: value.content,
        }
    }
}

impl From<UnderlineState> for ContentState {
    fn from(value: UnderlineState) -> Self {
        Self {
            content: value.content,
        }
    }
}

impl From<TrailingState> for ContentState {
    fn from(value: TrailingState) -> Self {
        Self {
            content: value.content,
        }
    }
}

impl From<ContentState> for STXHeadingSubState {
    fn from(value: ContentState) -> Self {
        STXHeadingSubState::Content(value)
    }
}