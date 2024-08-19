use crate::unicode;

#[derive(Clone, Copy)]
pub enum LineEnding {
    LineFeed,
    CarriageReturn,
    CarriageReturnLineFeed,
}

impl LineEnding {
    pub fn to_string(self) -> String {
        match self {
            LineEnding::LineFeed => format!("{}", unicode::LINE_FEED),
            LineEnding::CarriageReturn => format!("{}", unicode::CARRIAGE_RETURN),
            LineEnding::CarriageReturnLineFeed => format!("{}{}", unicode::CARRIAGE_RETURN, unicode::LINE_FEED),
        }
    }
}