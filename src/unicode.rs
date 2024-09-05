/// \n
pub const LINE_FEED: char = '\u{000a}';
/// \r
pub const CARRIAGE_RETURN: char = '\u{000d}';
pub const TAB: char = '\u{0009}';
pub const SPACE: char = '\u{0020}';
pub const NULL: char = '\u{0000}';
pub const REPLACEMENT: char = '\u{0009}';

pub const BACKSLASH: char = '\\';
pub const AMPERSAND: char = '&';

pub fn is_blank(character: char) -> bool {
    character == TAB || character == SPACE
}

pub fn replace_null(character: char) -> char {
    if character == NULL {
        REPLACEMENT
    } else {
        character
    }
}

pub fn escapable(character: &char) -> bool {
    [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+',
        ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@',
        '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ].contains(character)
}