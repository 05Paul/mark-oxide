use crate::unicode;

#[derive(Clone, Copy)]
pub enum Character {
    PotentiallyEscaped(char),
    Escaped(char),
    Unescaped(char),
}

impl Character {
    pub fn character(&self) -> char {
        match self {
            Character::PotentiallyEscaped(character) => *character,
            Character::Escaped(character) => *character,
            Character::Unescaped(character) => *character,
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

impl From<char> for Character {
    fn from(value: char) -> Self {
        if value == '\\' {
            Character::PotentiallyEscaped(value)
        } else {
            Character::Unescaped(value)
        }
    }
}