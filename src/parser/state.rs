use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::default::DefaultState;
use crate::parser::state::fenced_code_block::FencedCodeBlockState;
use crate::parser::state::indented_code_block::IndentedCodeBlockState;
use crate::parser::state::line_ending::LineEndingState;
use crate::parser::state::potential::PotentialState;
use crate::parser::state::potential_escape::PotentialEscapeState;
use crate::parser::state::setext_heading::SetextHeadingState;
use crate::parser::state::thematic_break::ThematicBreakState;
use crate::unicode;

mod potential_escape;
mod default;
mod potential;
mod thematic_break;
mod line_ending;
mod atx_heading;
mod indented_code_block;
mod setext_heading;
mod fenced_code_block;

pub trait Transition {
    fn transition(self, character: Character) -> Action;
    fn end_line(self, line_ending: LineEnding) -> Action;
    fn end(self) -> Action;
}

pub trait SubTransition: Transition {
    fn is_start(value: Character) -> bool;
}

#[derive(Clone, Copy)]
pub enum LineEnding {
    LineFeed,
    CarriageReturn,
    CarriageReturnLineFeed,
}

impl Into<String> for LineEnding {
    fn into(self) -> String {
        match self {
            LineEnding::LineFeed => format!("{}", unicode::LINE_FEED),
            LineEnding::CarriageReturn => format!("{}", unicode::CARRIAGE_RETURN),
            LineEnding::CarriageReturnLineFeed => format!("{}{}", unicode::CARRIAGE_RETURN, unicode::LINE_FEED),
        }
    }
}

#[derive(Clone)]
pub enum State {
    Default(DefaultState),
    ATXHeading(ATXHeadingState),
    SetextHeading(SetextHeadingState),
    ThematicBreak(ThematicBreakState),
    IndentedCodeBlock(IndentedCodeBlockState),
    FencedCodeBlock(FencedCodeBlockState),
    Potential(PotentialState),
    PotentialEscape(PotentialEscapeState),
    LineEnding(LineEndingState),
}

impl Default for State {
    fn default() -> Self {
        State::Default(DefaultState::default())
    }
}

impl Transition for State {
    fn transition(self, character: Character) -> Action {
        if let Ok(state) = LineEndingState::new(character.character(), &self) {
            return Action::Pass(
                State::LineEnding(state)
            );
        }

        if PotentialEscapeState::is_start(character) {
            return Action::Pass(
                State::PotentialEscape(PotentialEscapeState::new(self)),
            );
        }

        match self {
            State::Default(state) => state.transition(character),
            State::ATXHeading(state) => state.transition(character),
            State::SetextHeading(state) => state.transition(character),
            State::ThematicBreak(state) => state.transition(character),
            State::IndentedCodeBlock(state) => state.transition(character),
            State::FencedCodeBlock(state) => state.transition(character),
            State::Potential(state) => state.transition(character),
            State::PotentialEscape(state) => state.transition(character),
            State::LineEnding(state) => state.transition(character),
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Action {
        match self {
            State::Default(state) => state.end_line(line_ending),
            State::ATXHeading(state) => state.end_line(line_ending),
            State::SetextHeading(state) => state.end_line(line_ending),
            State::ThematicBreak(state) => state.end_line(line_ending),
            State::IndentedCodeBlock(state) => state.end_line(line_ending),
            State::FencedCodeBlock(state) => state.end_line(line_ending),
            State::Potential(state) => state.end_line(line_ending),
            State::PotentialEscape(state) => state.end_line(line_ending),
            State::LineEnding(state) => state.end_line(line_ending),
        }
    }

    fn end(self) -> Action {
        match self {
            State::Default(state) => state.end(),
            State::ATXHeading(state) => state.end(),
            State::SetextHeading(state) => state.end(),
            State::ThematicBreak(state) => state.end(),
            State::IndentedCodeBlock(state) => state.end(),
            State::FencedCodeBlock(state) => state.end(),
            State::Potential(state) => state.end(),
            State::PotentialEscape(state) => state.end(),
            State::LineEnding(state) => state.end(),
        }
    }
}