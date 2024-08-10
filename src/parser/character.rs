use crate::error::Error;
use crate::parser::state::LineEnding;
use crate::unicode;

#[derive(Clone)]
pub enum Character {
    Escaped(char),
    Unescaped(char),
    Reference {
        character: String,
        raw: String,
    },
}

impl Character {
    pub fn new_unescaped(character: char) -> Result<Self, Error> {
        if unicode::escapable(&character) {
            Ok(Character::Escaped(character))
        } else {
            Err(Error::Conversion)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Character::Escaped(character) |
            Character::Unescaped(character) => character.to_string(),
            Character::Reference { character, .. } => character.clone(),
        }
    }

    pub fn to_raw_string(&self) -> String {
        match self {
            Character::Escaped(character) |
            Character::Unescaped(character) => character.to_string(),
            Character::Reference { raw, .. } => raw.clone(),
        }
    }

    pub fn is_blank(&self) -> bool {
        match self {
            Character::Unescaped(char) => unicode::is_blank(*char),
            _ => false,
        }
    }

    pub fn space_count(&self) -> usize {
        match self {
            Character::Unescaped(unicode::SPACE) => 1,
            Character::Unescaped(unicode::TAB) => 4,
            _ => 0,
        }
    }
}

pub enum InvalidCharacterCategoryError {
    Escape,
    Reference,
    LineEnding(LineEnding),
}

impl TryFrom<char> for Character {
    type Error = InvalidCharacterCategoryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == unicode::BACKSLASH {
            Err(InvalidCharacterCategoryError::Escape)
        } else if value == unicode::AMPERSAND {
            Err(InvalidCharacterCategoryError::Reference)
        } else if value == unicode::LINE_FEED {
            Err(InvalidCharacterCategoryError::LineEnding(LineEnding::LineFeed))
        } else if value == unicode::CARRIAGE_RETURN {
            Err(InvalidCharacterCategoryError::LineEnding(LineEnding::CarriageReturn))
        } else {
            Ok(Character::Unescaped(value))
        }
    }
}

impl TryFrom<String> for Character {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with(unicode::AMPERSAND) {
            return Err(Error::Conversion);
        }

        if (value.starts_with("&#X") || value.starts_with("&#x")) && value.len() < 10 {
            let character = parse_numeric_reference(&value, 16)?;

            return Ok(
                Character::Reference {
                    character: character.to_string(),
                    raw: value,
                }
            );
        }

        if value.starts_with("&#") && value.len() < 10 {
            let character = parse_numeric_reference(&value, 10)?;

            return Ok(
                Character::Reference {
                    character: character.to_string(),
                    raw: value,
                }
            );
        }

        todo!()
    }
}

fn parse_numeric_reference(value: &str, radix: u32) -> Result<char, Error> {
    let character = u32::from_str_radix(
        &value.chars()
            .skip(2)
            .collect::<String>(),
        radix)
        .map_err(|_| Error::Conversion)
        .map(|value| char::from_u32(value)
            .unwrap_or(unicode::REPLACEMENT)
        )?;

    Ok(
        unicode::replace_null(character)
    )
}