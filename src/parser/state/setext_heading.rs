use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::parser::state::default::DefaultState;
use crate::unicode;

const LEVEL1: char = '=';
const LEVEL2: char = '-';

#[derive(Clone)]
pub struct SetextHeadingState {
    content: String,
    underline: Option<char>,
    leading_spaces: usize,
    trail: bool,
    line_break: bool,
}

impl SetextHeadingState {
    pub fn new(character: Character) -> Self {
        Self {
            content: character.character().to_string(),
            underline: None,
            leading_spaces: 0,
            trail: false,
            line_break: false,
        }
    }
}

impl Transition for SetextHeadingState {
    fn transition(self, character: Character) -> Action {
        match (self.line_break, self.underline, character, self.leading_spaces, self.trail) {
            (_, None, Character::Unescaped(LEVEL1 | LEVEL2), _, _) => Action::Pass(
                State::SetextHeading(
                    Self {
                        underline: Some(character.character()),
                        ..self
                    }
                )
            ),
            (true, None, Character::Unescaped(unicode::SPACE), 0..=2, _) => Action::Pass(
                State::SetextHeading(
                    Self {
                        leading_spaces: self.leading_spaces + 1,
                        ..self
                    }
                )
            ),
            (true, None, Character::Unescaped(unicode::SPACE), 3, _) => Action::Pass(
                State::Default(
                    DefaultState::new(4)
                )
            ),
            (_, None, Character::Unescaped(char) | Character::Escaped(char), _, _) => Action::Pass(
                State::SetextHeading(
                    Self {
                        content: self.content + char.to_string().as_str(),
                        ..self
                    }
                )
            ),
            (true, Some(LEVEL1), Character::Unescaped(LEVEL1), _, false) |
            (true, Some(LEVEL2), Character::Unescaped(LEVEL2), _, false) => Action::Pass(
                State::SetextHeading(
                    Self {
                        ..self
                    }
                )
            ),
            (true, Some(_), Character::Unescaped(unicode::SPACE), _, _) => Action::Pass(
                State::SetextHeading(
                    Self {
                        trail: true,
                        ..self
                    }
                )
            ),
            _ => Action::Dismiss,
        }
    }

    fn end_line(self, line_ending: LineEnding) -> Action {
        match (self.line_break, self.underline) {
            (true, Some(LEVEL1)) => Leaf::SetextHeading {
                level: 1,
                text: self.content.trim_end().to_string(),
            }.into_action(),
            (true, Some(LEVEL2)) => Leaf::SetextHeading {
                level: 2,
                text: self.content.trim_end().to_string(),
            }.into_action(),
            (true, None) => Action::Dismiss,
            (false, _) => Action::Pass(
                State::SetextHeading(
                    Self {
                        content: self.content + <LineEnding as Into<String>>::into(line_ending).as_str(),
                        underline: None,
                        leading_spaces: 0,
                        trail: false,
                        line_break: true,
                    }
                )
            ),
            _ => Action::Dismiss,
        }
    }

    fn end(self) -> Action {
        match (self.line_break, self.underline) {
            (true, Some(LEVEL1)) => Leaf::SetextHeading {
                level: 1,
                text: self.content.trim_end().to_string(),
            }.into_action(),
            (true, Some(LEVEL2)) => Leaf::SetextHeading {
                level: 2,
                text: self.content.trim_end().to_string(),
            }.into_action(),
            _ => Action::Dismiss,
        }
    }
}

impl SubTransition for SetextHeadingState {
    fn is_start(_: Character) -> bool {
        true
    }
}