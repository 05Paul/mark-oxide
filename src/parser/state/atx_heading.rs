use crate::error::Error;
use crate::parser::action::Action;
use crate::parser::character::Character;
use crate::parser::document::leaf::Leaf;
use crate::parser::state::{LineEnding, State, SubTransition, Transition};
use crate::unicode;

pub const HASHTAG: char = '#';

#[derive(Clone)]
pub struct ATXHeadingState {
    pub character_count: usize,
    pub text: String,
    temp: String,
}

impl TryFrom<Character> for ATXHeadingState {
    type Error = Error;

    fn try_from(value: Character) -> Result<Self, Self::Error> {
        if Self::is_start(value) {
            Ok(
                Self {
                    character_count: 1,
                    text: "".to_string(),
                    temp: "".to_string(),
                }
            )
        } else {
            Err(Error::StartState)
        }
    }
}

impl Transition for ATXHeadingState {
    fn transition(self, character: Character) -> Action {
        match (character.clone(), self.character_count) {
            // Case: hashtag
            (Character::Unescaped(HASHTAG), 0..=5) => {
                let length = self.text.len() + self.temp.len();
                let last_temp = self.temp.chars().last();
                let contains_hashtag = self.temp.contains(HASHTAG);

                match (length, last_temp, contains_hashtag) {
                    // Case: trailing whitespace
                    (1.., Some(unicode::SPACE | unicode::TAB), true) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                text: self.text + &*self.temp,
                                temp: character.to_string(),
                                ..self
                            })
                        ),
                    // Case: trailing whitespace or hashtag
                    (1.., Some(unicode::SPACE | unicode::TAB | HASHTAG), _) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                temp: self.temp + character.to_string().as_str(),
                                ..self
                            })
                        ),
                    // Case: trailing character
                    (1.., _, _) =>
                        Action::Pass(
                            State::ATXHeading(Self {
                                text: self.text + character.to_string().as_str(),
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
            (Character::Unescaped(unicode::SPACE), 1..) =>
                Action::Pass(
                    State::ATXHeading(Self {
                        temp: self.temp + character.to_string().as_str(),
                        ..self
                    })
                ),

            // Case: tab
            (Character::Unescaped(unicode::TAB), 1..) => Action::Pass(
                State::ATXHeading(Self {
                    temp: self.temp + character.to_string().as_str(),
                    ..self
                })
            ),
            // Case: non whitespace character after first '#'
            (_, 1..) => Action::Pass(
                State::ATXHeading(Self {
                    text: self.text + self.temp.to_string().as_str() +
                        character.to_string().as_str(),
                    temp: "".into(),
                    ..self
                })
            ),
            // Case: dismiss
            _ => Action::Dismiss
        }
    }

    fn end_line(self, _: LineEnding) -> Action {
        self.end()
    }

    fn end(self) -> Action {
        let last = self.text.chars().last();
        match (self.character_count, last) {
            (0, _) => Action::Dismiss,
            _ => Leaf::AtxHeading {
                level: self.character_count,
                text: self.text.trim().to_string(),
            }.into_action(),
        }
    }
}

impl SubTransition for ATXHeadingState {
    fn is_start(value: Character) -> bool {
        match value {
            Character::Unescaped(HASHTAG) => true,
            _ => false,
        }
    }
}