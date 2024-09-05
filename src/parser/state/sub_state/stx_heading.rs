mod content;
mod newline;
mod underline;
mod trailing;

use crate::error::Error;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::state::handler::Handler;
use crate::parser::state::sub_state::stx_heading::content::ContentState;
use crate::parser::state::sub_state::stx_heading::newline::NewlineState;
use crate::parser::state::sub_state::stx_heading::trailing::TrailingState;
use crate::parser::state::sub_state::stx_heading::underline::UnderlineState;
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};

const LEVEL1: char = '=';
const LEVEL2: char = '-';

#[derive(Clone, Copy, PartialEq)]
pub enum UnderlineCharacter {
    Level1,
    Level2,
}

impl UnderlineCharacter {
    pub fn level(&self) -> usize {
        match self {
            UnderlineCharacter::Level1 => 1,
            UnderlineCharacter::Level2 => 2,
        }
    }

    fn character(&self) -> char {
        match self {
            UnderlineCharacter::Level1 => LEVEL1,
            UnderlineCharacter::Level2 => LEVEL2,
        }
    }
}

impl TryFrom<&Character> for UnderlineCharacter {
    type Error = Error;

    fn try_from(value: &Character) -> Result<Self, Self::Error> {
        match value {
            Character::Unescaped(LEVEL1) => Ok(UnderlineCharacter::Level1),
            Character::Unescaped(LEVEL2) => Ok(UnderlineCharacter::Level2),
            _ => Err(Error::Conversion)
        }
    }
}

pub type STXHeadingState = Handler<SubState, STXHeadingSubState>;

impl From<Character> for STXHeadingState {
    fn from(value: Character) -> Self {
        Handler::new(STXHeadingSubState::from(value))
    }
}

impl From<STXHeadingState> for SubState {
    fn from(value: STXHeadingState) -> Self {
        SubState::STXHeading(value)
    }
}

pub enum STXHeadingSubState {
    Content(ContentState),
    Newline(NewlineState),
    Underline(UnderlineState),
    Trailing(TrailingState),
}

impl STXHeadingSubState {
    pub fn complete(state: impl Into<STXHeadingSubState>) -> NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>> {
        let state = state.into();
        let (content, underline) = match state {
            STXHeadingSubState::Underline(state) => (state.content, state.underline_character),
            STXHeadingSubState::Trailing(state) => (state.content, state.underline_character),
            _ => unreachable!()
        };

        NonDeterministicTransitionEffect::complete::<Block>(Leaf::SetextHeading {
            level: underline.level(),
            text: content.trim_end().to_string(),
        })
    }
}

impl Transition for STXHeadingSubState {
    type Effect = NonDeterministicTransitionEffect<STXHeadingSubState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        match self {
            STXHeadingSubState::Content(state) => state.transition(character),
            STXHeadingSubState::Newline(state) => state.transition(character),
            STXHeadingSubState::Underline(state) => state.transition(character),
            STXHeadingSubState::Trailing(state) => state.transition(character),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        match self {
            STXHeadingSubState::Content(state) => state.end_line(line_ending),
            STXHeadingSubState::Newline(state) => state.end_line(line_ending),
            STXHeadingSubState::Underline(state) => state.end_line(line_ending),
            STXHeadingSubState::Trailing(state) => state.end_line(line_ending),
        }
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        match self {
            STXHeadingSubState::Content(state) => state.end(),
            STXHeadingSubState::Newline(state) => state.end(),
            STXHeadingSubState::Underline(state) => state.end(),
            STXHeadingSubState::Trailing(state) => state.end(),
        }
    }
}

impl From<Character> for STXHeadingSubState {
    fn from(value: Character) -> Self {
        ContentState::new(value).into()
    }
}