use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::atx_heading::ATXHeadingState;
use crate::parser::state::default::DefaultState;
use crate::parser::state::line_ending::LineEndingState;
use crate::parser::state::potential::PotentialState;
use crate::parser::state::potential_escape::PotentialEscapeState;
use crate::parser::state::thematic_break::ThematicBreakState;

mod potential_escape;
mod default;
mod potential;
mod thematic_break;
mod line_ending;
mod atx_heading;

pub trait Transition {
    fn transition(self, character: Character) -> Action;
    fn end(self) -> Action;
}

pub trait SubTransition: Transition {
    fn is_start(value: Character) -> bool;
}

// Todo: implement SetextHeading (requires paragraph)
// Todo: implement a generic solution for leading spaces
#[derive(Clone)]
pub enum State {
    Default(DefaultState),
    ATXHeading(ATXHeadingState),
    ThematicBreak(ThematicBreakState),
    Potential(PotentialState),
    PotentialEscape(PotentialEscapeState),
    LineEnding(LineEndingState),
}

impl Default for State {
    fn default() -> Self {
        State::Default(DefaultState)
    }
}

impl Transition for State {
    fn transition(self, character: Character) -> Action {
        if LineEndingState::is_start(character) {
            return Action::Pass(
                State::LineEnding(LineEndingState::new(character.character(), self))
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
            State::ThematicBreak(state) => state.transition(character),
            State::Potential(state) => state.transition(character),
            State::PotentialEscape(state) => state.transition(character),
            State::LineEnding(state) => state.transition(character),
        }
    }

    fn end(self) -> Action {
        match self {
            State::Default(state) => state.end(),
            State::ATXHeading(state) => state.end(),
            State::ThematicBreak(state) => state.end(),
            State::Potential(state) => state.end(),
            State::PotentialEscape(state) => state.end(),
            State::LineEnding(state) => state.end(),
        }
    }
}