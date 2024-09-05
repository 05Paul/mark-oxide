mod opening;
mod info;
mod opening_trail;
mod newline;
mod content;
mod closing;

use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::handler::Handler;
use crate::parser::state::sub_state::fenced_code_block::closing::ClosingState;
use crate::parser::state::sub_state::fenced_code_block::content::ContentState;
use crate::parser::state::sub_state::fenced_code_block::info::InfoStringState;
use crate::parser::state::sub_state::fenced_code_block::newline::NewlineState;
use crate::parser::state::sub_state::fenced_code_block::opening::OpeningState;
use crate::parser::state::sub_state::fenced_code_block::opening_trail::OpeningTrailingState;
use crate::parser::state::sub_state::SubState;
use crate::parser::state::LineEnding;
use crate::parser::transition::{Transition, TransitionEffect};

pub(self) const BACKTICK: char = '`';
pub(self) const TILDE: char = '~';

#[derive(Clone, Copy, PartialEq)]
pub enum FenceCharacter {
    Backtick,
    Tilde,
}

impl FenceCharacter {
    pub fn character(&self) -> char {
        match self {
            FenceCharacter::Backtick => BACKTICK,
            FenceCharacter::Tilde => TILDE,
        }
    }

    pub fn repeat(&self, n: usize) -> String {
        self.character().to_string().repeat(n)
    }
}

impl TryFrom<&Character> for FenceCharacter {
    type Error = Error;

    fn try_from(value: &Character) -> Result<Self, Self::Error> {
        match value {
            Character::Unescaped(BACKTICK) => Ok(FenceCharacter::Backtick),
            Character::Unescaped(TILDE) => Ok(FenceCharacter::Tilde),
            _ => Err(Error::Conversion),
        }
    }
}

pub type FencedCodeBlockState = Handler<SubState, FencedCodeBlockSubState>;

impl TryFrom<(usize, Character)> for FencedCodeBlockState {
    type Error = Error;

    fn try_from((indentation, character): (usize, Character)) -> Result<Self, Error> {
        Ok(
            FencedCodeBlockState::new(
                OpeningState::new(indentation, character)?
                    .into()
            )
        )
    }
}

impl From<FencedCodeBlockState> for SubState {
    fn from(value: FencedCodeBlockState) -> Self {
        value.state().into()
    }
}

pub enum FencedCodeBlockSubState {
    Opening(OpeningState),
    Info(InfoStringState),
    OpeningTrail(OpeningTrailingState),
    Newline(NewlineState),
    Content(ContentState),
    Closing(ClosingState),
}

impl Transition for FencedCodeBlockSubState {
    type Effect = NonDeterministicTransitionEffect<FencedCodeBlockSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            FencedCodeBlockSubState::Opening(state) => state.transition(character),
            FencedCodeBlockSubState::Info(state) => state.transition(character),
            FencedCodeBlockSubState::OpeningTrail(state) => state.transition(character),
            FencedCodeBlockSubState::Newline(state) => state.transition(character),
            FencedCodeBlockSubState::Content(state) => state.transition(character),
            FencedCodeBlockSubState::Closing(state) => state.transition(character),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            FencedCodeBlockSubState::Opening(state) => state.end_line(line_ending),
            FencedCodeBlockSubState::Info(state) => state.end_line(line_ending),
            FencedCodeBlockSubState::OpeningTrail(state) => state.end_line(line_ending),
            FencedCodeBlockSubState::Newline(state) => state.end_line(line_ending),
            FencedCodeBlockSubState::Content(state) => state.end_line(line_ending),
            FencedCodeBlockSubState::Closing(state) => state.end_line(line_ending),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            FencedCodeBlockSubState::Opening(state) => state.end(),
            FencedCodeBlockSubState::Info(state) => state.end(),
            FencedCodeBlockSubState::OpeningTrail(state) => state.end(),
            FencedCodeBlockSubState::Newline(state) => state.end(),
            FencedCodeBlockSubState::Content(state) => state.end(),
            FencedCodeBlockSubState::Closing(state) => state.end(),
        }
    }
}

impl From<FencedCodeBlockSubState> for SubState {
    fn from(value: FencedCodeBlockSubState) -> Self {
        SubState::FencedCodeBlock(FencedCodeBlockState::new(value))
    }
}