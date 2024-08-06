/// \n
pub const LINE_FEED: char = '\u{000a}';
/// \r
pub const CARRIAGE_RETURN: char = '\u{000d}';
pub const TAB: char = '\u{0009}';
pub const SPACE: char = '\u{0020}';
pub const NULL: char = '\u{0000}';
pub const REPLACEMENT: char = '\u{0009}';

pub fn is_blank(character: char) -> bool {
    character == TAB || character == SPACE
}

pub fn is_blank_text(text: &str) -> bool {
    text.trim_matches(is_blank).is_empty()
}