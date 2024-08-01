use crate::error::Error;
use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::unicode;

#[derive(Clone)]
pub struct LineEndingState {
    previous_line_ending: LineEnding,
    previous_state: Box<State>,
}

impl LineEndingState {
    pub fn new(character: char, state: &State) -> Result<Self, Error> {
        let previous_line_ending = match character {
            unicode::LINE_FEED => LineEnding::LineFeed,
            unicode::CARRIAGE_RETURN => LineEnding::CarriageReturn,
            _ => return Err(Error::StartStateError),
        };
        Ok(
            Self {
                previous_line_ending,
                previous_state: Box::new(state.clone()),
            }
        )
    }
}

impl Transition for LineEndingState {
    fn transition(self, character: Character) -> Action {
        match (self.previous_line_ending, character.character()) {
            (LineEnding::CarriageReturn, unicode::LINE_FEED) => self.previous_state.end_line(LineEnding::CarriageReturnLineFeed),
            (previous_line_ending, _) => {
                match self.previous_state.end_line(previous_line_ending) {
                    Action::Pass(state) => state.transition(character),
                    Action::Complete(block) => Action::Complete(block)
                        .merge(
                            State::default()
                                .transition(character)
                        ),
                    Action::Dismiss => State::default().transition(character),
                    Action::Bi { .. } => unreachable!(),
                }
            }
        }
    }

    fn end(self) -> Action {
        self.previous_state.end()
    }

    fn end_line(self, line_ending: LineEnding) -> Action {
        let action = self.previous_state.end_line(self.previous_line_ending);

        match action {
            Action::Pass(state) => state.end_line(line_ending),
            _ => action,
        }
    }
}

impl SubTransition for LineEndingState {
    fn is_start(value: Character) -> bool {
        value.character() == unicode::LINE_FEED || value.character() == unicode::CARRIAGE_RETURN
    }
}