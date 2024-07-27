use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{State, SubTransition, Transition};
use crate::unicode;

#[derive(Clone)]
pub struct ATXHeadingState {
    pub character_count: usize,
    leading_spaces: usize,
    pub text: String,
    temp: String,
}

impl ATXHeadingState {
    pub fn new(character: Character) -> Self {
        match character {
            Character::Unescaped('#') => Self {
                character_count: 1,
                leading_spaces: 0,
                text: "".to_string(),
                temp: "".to_string(),
            },
            _ => Self {
                character_count: 0,
                leading_spaces: 1,
                text: "".to_string(),
                temp: "".to_string(),
            },
        }
    }
}

impl Transition for ATXHeadingState {
    fn transition(self, character: Character) -> Action {
        match (character, self.character_count, self.leading_spaces) {
            // Case: hashtag
            (Character::Unescaped('#'), 0..=5, _) => {
                let length = self.text.len() + self.temp.len();
                let last_temp = self.temp.chars().last();
                let contains_hashtag = self.temp.contains("#");

                match (length, last_temp, contains_hashtag) {
                    // Case: trailing whitespace
                    (1.., Some(unicode::SPACE | unicode::TAB), true) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                text: self.text + &*self.temp,
                                temp: character.character().to_string(),
                                ..self
                            })
                        ),
                    // Case: trailing whitespace or hashtag
                    (1.., Some(unicode::SPACE | unicode::TAB | '#'), _) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                temp: self.temp + character.character().to_string().as_str(),
                                ..self
                            })
                        ),
                    // Case: trailing character
                    (1.., _, _) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                text: self.text + character.character().to_string().as_str(),
                                ..self
                            })
                        ),
                    // Case: content character
                    _ => Action::Pass(
                        State::ATXHeading(Self {
                            character_count: self.character_count + 1,
                            ..self
                        })
                    )
                }
            }
            // Case: non-leading space
            (Character::Unescaped(unicode::SPACE), 1.., _) =>
                Action::Pass(
                    State::ATXHeading(Self {
                        temp: self.temp + character.character().to_string().as_str(),
                        ..self
                    })
                ),
            // Case: leading space
            (Character::Unescaped(unicode::SPACE), _, 0..=2) => Action::Pass(
                State::ATXHeading(Self {
                    leading_spaces: self.leading_spaces + 1,
                    ..self
                })
            ),
            // Case: tab
            (Character::Unescaped(unicode::TAB), 1.., _) => Action::Pass(
                State::ATXHeading(Self {
                    temp: self.temp + character.character().to_string().as_str(),
                    ..self
                })
            ),
            // Case: non whitespace character after first '#'
            (_, 1.., _) => Action::Pass(
                State::ATXHeading(Self {
                    text: self.text + self.temp.to_string().as_str() +
                        character.character().to_string().as_str(),
                    temp: "".into(),
                    ..self
                })
            ),
            // Case: dismiss
            _ => Action::Dismiss
        }
    }

    fn end(self) -> Action {
        let last = self.text.chars().last();
        match (self.character_count, last) {
            (0, _) => Action::Dismiss,
            _ => Action::Complete(
                Block::Leaf(
                    Leaf::AtxHeading {
                        level: self.character_count,
                        text: self.text.trim().to_string(),
                    }
                )
            ),
        }
    }
}

impl SubTransition for ATXHeadingState {
    fn is_start(value: Character) -> bool {
        match value {
            Character::Unescaped('#' | unicode::SPACE) => true,
            _ => false,
        }
    }
}