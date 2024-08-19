use crate::parser::character::Character;
use crate::parser::line_ending::LineEnding;

pub trait TransitionEffect {
    type State;
    type Outcome;

    fn content(self) -> (Self::State, Self::Outcome);
}

pub trait Transition {
    type Effect: TransitionEffect;

    fn transition(self, character: Character) -> Self::Effect;
    fn end_line(self, line_ending: LineEnding) -> Self::Effect;
    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome ;
}

/*
pub trait SubTransition {
    type TransitionResult;
    type EndResult;

    fn transition(&mut self, character: Character) -> Self::TransitionResult;
    fn end_line(&mut self, line_ending: LineEnding) -> Self::TransitionResult;
    fn end(self) -> Self::EndResult;
}
 */