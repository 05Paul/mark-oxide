use crate::state::default::DefaultState;
use crate::state::line_ending::LineEndingState;
use crate::state::potential::PotentialState;
use crate::state::potential_escape::PotentialEscapeState;
use crate::state::thematic_break::ThematicBreakState;

mod potential_escape;
mod default;
mod potential;
mod thematic_break;
mod line_ending;

pub trait Transition {
    fn transition(self, character: Character) -> (State, Action);
    fn end(self) -> (State, Action);
}

pub trait SubTransition: Transition {
    fn is_start(value: Character) -> bool;
}

pub enum State {
    Default(DefaultState),
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
    fn transition(self, character: Character) -> (State, Action) {
        if LineEndingState::is_start(character) {
            return (
                State::LineEnding(LineEndingState::new(character.character(), self)),
                Action::Pass,
            );
        }

        match self {
            State::Default(state) => state.transition(character),
            State::ThematicBreak(state) => state.transition(character),
            State::Potential(state) => state.transition(character),
            State::PotentialEscape(state) => state.transition(character),
            State::LineEnding(state) => state.transition(character),
        }
    }

    fn end(self) -> (State, Action) {
        match self {
            State::Default(state) => state.end(),
            State::ThematicBreak(state) => state.end(),
            State::Potential(state) => state.end(),
            State::PotentialEscape(state) => state.end(),
            State::LineEnding(state) => state.end(),
        }
    }
}

pub enum Action {
    Pass,
    Dismiss,
    Complete(State),
}

#[derive(Clone, Copy)]
pub enum Character {
    Escaped(char),
    Unescaped(char),
}

impl Character {
    fn character(&self) -> char {
        match self {
            Character::Escaped(character) => *character,
            Character::Unescaped(character) => *character,
        }
    }
}

impl Character {
    pub fn new(character: char) -> Self {
        Character::Unescaped(character)
    }
}