use crate::parser::character::Character;
use crate::parser::line_ending::LineEnding;

pub trait CharacterTransitionHandler {
    type State;
    type TransitionResult;

    fn transition(&mut self, character: char) -> Self::TransitionResult;
    fn end(self) -> Self::TransitionResult;
}

pub trait CharacterSubTransition {
    type Handler: CharacterTransitionHandler;
    fn transition(&self, character: char) -> CharacterStateTransition<<Self::Handler as CharacterTransitionHandler>::State, <Self::Handler as CharacterTransitionHandler>::TransitionResult>;
    fn end(&self) -> <Self::Handler as CharacterTransitionHandler>::TransitionResult;
}

pub struct CharacterStateTransition<State, TransitionResult> {
    pub state: State,
    pub result: TransitionResult,
}

impl<State, TransitionResult> CharacterStateTransition<State, TransitionResult> {
    pub fn new(state: impl Into<State>, result: TransitionResult) -> Self {
        Self {
            state: state.into(),
            result,
        }
    }

    pub fn transition_into<NewState: Into<State> + Default>(result: TransitionResult) -> Self {
        Self {
            state: NewState::default().into(),
            result,
        }
    }

    pub fn transition_into_from<NewState: Into<State>>(value: impl Into<NewState>, result: TransitionResult) -> Self {
        Self {
            state: value.into().into(),
            result,
        }
    }
}

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