use crate::parser::character::Character;
use crate::parser::state::LineEnding;

pub enum PositionedLineEnding {
    Before(LineEnding),
    After(LineEnding)
}

pub struct CharacterTransitionResult {
    pub line_ending: Option<PositionedLineEnding>,
    pub characters: Vec<Character>,
}

impl CharacterTransitionResult {
    pub fn line_ending_before(line_ending: LineEnding) -> Self {
        Self {
            line_ending: Some(PositionedLineEnding::Before(line_ending)),
            characters: vec![],
        }
    }

    pub fn characters(characters: Vec<Character>) -> Self {
        Self {
            line_ending: None,
            characters,
        }
    }

    pub fn with_line_ending(mut self, line_ending: PositionedLineEnding) -> Self {
        self.line_ending = Some(line_ending);
        self
    }

    pub fn with_characters(mut self, characters: Vec<Character>) -> Self {
        self.characters = characters;
        self
    }
}

impl Default for CharacterTransitionResult {
    fn default() -> Self {
        Self {
            line_ending: None,
            characters: vec![],
        }
    }
}