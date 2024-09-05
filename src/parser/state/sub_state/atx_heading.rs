mod opening;
mod leading;
mod content;
mod trailing;
mod closing;

use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::handler::Handler;
use crate::parser::state::sub_state::atx_heading::closing::PotentiallyClosingState;
use crate::parser::state::sub_state::atx_heading::content::ContentState;
use crate::parser::state::sub_state::atx_heading::leading::LeadingWhitespaceState;
use crate::parser::state::sub_state::atx_heading::opening::OpeningSequenceState;
use crate::parser::state::sub_state::atx_heading::trailing::PotentiallyTrailingState;
use crate::parser::state::sub_state::SubState;
use crate::parser::state::LineEnding;
use crate::parser::transition::{Transition, TransitionEffect};

pub const HASHTAG: char = '#';

pub type ATXHeadingState = Handler<SubState, ATXHeadingSubState>;

impl TryFrom<Character> for ATXHeadingState {
    type Error = Error;

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        if let Character::Unescaped(HASHTAG) = value {
            Ok(
                Handler::new(ATXHeadingSubState::default())
            )
        } else {
            Err(Error::StartState)
        }
    }
}

impl From<ATXHeadingState> for SubState {
    fn from(value: ATXHeadingState) -> Self {
        SubState::ATXHeading(value)
    }
}

pub enum ATXHeadingSubState {
    Opening(OpeningSequenceState),
    LeadingWhitespace(LeadingWhitespaceState),
    Content(ContentState),
    PotentiallyTrailing(PotentiallyTrailingState),
    PotentiallyClosing(PotentiallyClosingState),
}

impl ATXHeadingSubState {
    fn level(&self) -> usize {
        match self {
            ATXHeadingSubState::Opening(state) => state.level,
            ATXHeadingSubState::LeadingWhitespace(state) => state.level,
            ATXHeadingSubState::Content(state) => state.level,
            ATXHeadingSubState::PotentiallyTrailing(state) => state.level,
            ATXHeadingSubState::PotentiallyClosing(state) => state.level,
        }
    }

    fn content(self) -> String {
        match self {
            ATXHeadingSubState::Content(state) => state.content,
            ATXHeadingSubState::PotentiallyTrailing(state) => state.content,
            ATXHeadingSubState::PotentiallyClosing(state) => state.content,
            _ => String::new(),
        }
    }

    fn complete(state: impl Into<ATXHeadingSubState>) -> NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>> {
        let state = state.into();
        NonDeterministicTransitionEffect::complete::<Block>(Leaf::AtxHeading {
            level: state.level(),
            text: state.content(),
        })
    }
}

impl Transition for ATXHeadingSubState {
    type Effect = NonDeterministicTransitionEffect<ATXHeadingSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            ATXHeadingSubState::Opening(state) => state.transition(character),
            ATXHeadingSubState::LeadingWhitespace(state) => state.transition(character),
            ATXHeadingSubState::Content(state) => state.transition(character),
            ATXHeadingSubState::PotentiallyTrailing(state) => state.transition(character),
            ATXHeadingSubState::PotentiallyClosing(state) => state.transition(character),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            ATXHeadingSubState::Opening(state) => state.end_line(line_ending),
            ATXHeadingSubState::LeadingWhitespace(state) => state.end_line(line_ending),
            ATXHeadingSubState::Content(state) => state.end_line(line_ending),
            ATXHeadingSubState::PotentiallyTrailing(state) => state.end_line(line_ending),
            ATXHeadingSubState::PotentiallyClosing(state) => state.end_line(line_ending),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            ATXHeadingSubState::Opening(state) => state.end(),
            ATXHeadingSubState::LeadingWhitespace(state) => state.end(),
            ATXHeadingSubState::Content(state) => state.end(),
            ATXHeadingSubState::PotentiallyTrailing(state) => state.end(),
            ATXHeadingSubState::PotentiallyClosing(state) => state.end(),
        }
    }
}

impl Default for ATXHeadingSubState {
    fn default() -> Self {
        ATXHeadingSubState::Opening(
            OpeningSequenceState {
                level: 1,
            },
        )
    }
}