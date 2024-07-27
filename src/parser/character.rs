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