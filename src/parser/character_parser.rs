use std::ops::Deref;
use crate::parser::action::Action;
use crate::parser::character_parser::carriage_return_state::CarriageReturnState;
use crate::parser::character_parser::character_transition_result::{CharacterTransitionResult, PositionedLineEnding};
use crate::parser::character_parser::default_state::DefaultState;
use crate::parser::character_parser::potential_character_reference::PotentialCharacterReferenceState;
use crate::parser::character_parser::potential_escape_state::PotentialEscapeState;
use crate::parser::document::Document;
use crate::parser::state::{State, Transition};

mod default_state;
mod potential_escape_state;
mod carriage_return_state;
mod potential_character_reference;
mod character_transition_result;

pub struct CharacterParser {
    document: Document,
    state: State,
    character_buffer: Vec<char>,
    internal_state_handler: CharacterParserStateHandler,
}

impl CharacterParser {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            state: Default::default(),
            character_buffer: Vec::new(),
            internal_state_handler: CharacterParserStateHandler(Default::default()),
        }
    }

    pub fn parse_character(&mut self, character: char) {
        let result = self.internal_state_handler.transition(character);

        self.state = Self::handle_transition_result(result, self.state.clone(), &mut self.document);
    }

    pub fn end_document(mut self) -> Document {
        let result = self.internal_state_handler.end();

        self.state = Self::handle_transition_result(result, self.state, &mut self.document);
        self.state = Self::handle_action(&mut self.document, self.state.end());

        self.document
    }

    fn handle_transition_result(result: CharacterTransitionResult, mut state: State, document: &mut Document) -> State {
        if let Some(PositionedLineEnding::Before(line_ending)) = result.line_ending {
            state = Self::handle_action(
                document,
                state.end_line(line_ending),
            );
        }

        for character in result.characters {
            state = Self::handle_action(document, state.transition(character));
        }

        if let Some(PositionedLineEnding::After(line_ending)) = result.line_ending {
            state = Self::handle_action(
                document,
                state.end_line(line_ending),
            );
        }

        state
    }

    fn handle_action(document: &mut Document, action: Action) -> State {
        match action {
            Action::Complete(block) => {
                document.push(block);
                State::default()
            }
            Action::Pass(state) => state,
            Action::Bi { first, second } => match (first.deref().clone(), second.deref().clone()) {
                (Action::Complete(block), Action::Pass(state)) => {
                    document.push(block);
                    state
                }
                _ => {
                    unreachable!("ONLY COMPLETE PASS ?");
                }
            }
            _ => State::default(),
        }
    }
}

struct StateTransition<State, TransitionResult> {
    state: State,
    result: TransitionResult,
}

impl<State, TransitionResult> StateTransition<State, TransitionResult> {
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

trait TransitHandler {
    type State;
    type TransitionResult;

    fn transition(&mut self, character: char) -> Self::TransitionResult;
    fn end(self) -> Self::TransitionResult;
}

trait SubStateTransit {
    type Handler: TransitHandler;
    fn transition(&self, character: char) -> StateTransition<<Self::Handler as TransitHandler>::State, <Self::Handler as TransitHandler>::TransitionResult>;
    fn end(&self) -> <Self::Handler as TransitHandler>::TransitionResult;
}

struct CharacterParserStateHandler(CharacterParserState);

enum CharacterParserState {
    Default(DefaultState),
    PotentialEscape(PotentialEscapeState),
    CarriageReturn(CarriageReturnState),
    PotentialCharacterReference(PotentialCharacterReferenceState),
}

impl TransitHandler for CharacterParserStateHandler {
    type State = CharacterParserState;
    type TransitionResult = CharacterTransitionResult;

    fn transition(&mut self, character: char) -> Self::TransitionResult {
        let transition = match &self.0 {
            CharacterParserState::Default(state) => state.transition(character),
            CharacterParserState::PotentialEscape(state) => state.transition(character),
            CharacterParserState::CarriageReturn(state) => state.transition(character),
            CharacterParserState::PotentialCharacterReference(state) => state.transition(character),
        };

        self.0 = transition.state;

        transition.result
    }

    fn end(self) -> Self::TransitionResult {
        match self.0 {
            CharacterParserState::Default(state) => state.end(),
            CharacterParserState::PotentialEscape(state) => state.end(),
            CharacterParserState::CarriageReturn(state) => state.end(),
            CharacterParserState::PotentialCharacterReference(state) => state.end(),
        }
    }
}

impl Default for CharacterParserState {
    fn default() -> Self {
        CharacterParserState::Default(DefaultState)
    }
}