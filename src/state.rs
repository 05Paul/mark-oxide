pub trait Transition {
    fn transition(self, character: Character) -> (State, Action);
    fn end(self) -> (State, Action);
}

pub enum State {
    Default
}

impl Default for State {
    fn default() -> Self {
        State::Default
    }
}

pub enum Action {}

pub enum Character {}