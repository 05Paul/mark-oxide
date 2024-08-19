use crate::parser::character_parser::carriage_return::CarriageReturnState;
use crate::parser::character_parser::character_transition::{CharacterSubTransition, CharacterTransitionHandler, CharacterTransitionResult, PositionedLineEnding};
use crate::parser::character_parser::default::DefaultState;
use crate::parser::character_parser::potential_character_reference::PotentialCharacterReferenceState;
use crate::parser::character_parser::potential_escape::PotentialEscapeState;
use crate::parser::document::block::Block;
use crate::parser::document::Document;
use crate::parser::state::StateHandler;
use crate::parser::transition::{Transition, TransitionEffect};

mod default;
mod potential_escape;
mod carriage_return;
mod potential_character_reference;
mod character_transition;

pub struct CharacterParser {
    document: Document,
    state: StateHandler,
    internal_state_handler: CharParserStateHandler,
}

impl CharacterParser {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            state: Default::default(),
            internal_state_handler: CharParserStateHandler(Default::default()),
        }
    }

    pub fn parse_character(&mut self, character: char) {
        let result = self.internal_state_handler.transition(character);

        Self::handle_transition_result(result, &mut self.state, &mut self.document);
    }

    pub fn end_document(mut self) -> Document {
        let result = self.internal_state_handler.end();

        Self::handle_transition_result(result, &mut self.state, &mut self.document);
        Self::handle_result(&mut self.document, self.state.end());

        self.document
    }

    fn handle_transition_result(result: CharacterTransitionResult, state: &mut StateHandler, document: &mut Document) {
        if let Some(PositionedLineEnding::Before(line_ending)) = result.line_ending {
            let (_, block) = state.end_line(line_ending).content();
            Self::handle_result(
                document,
                block,
            );
        }

        for character in result.characters {
            let (_, block) = state.transition(character).content();
            Self::handle_result(
                document,
                block
            );
        }

        if let Some(PositionedLineEnding::After(line_ending)) = result.line_ending {
            let (_, block) = state.end_line(line_ending).content();
            Self::handle_result(
                document,
                block,
            );
        }
    }

    fn handle_result(document: &mut Document, block: Option<Block>) {
        if let Some(block) = block {
            document.push(block);
        }
    }
}

pub struct CharParserStateHandler(CharacterParserState);

pub enum CharacterParserState {
    Default(DefaultState),
    PotentialEscape(PotentialEscapeState),
    CarriageReturn(CarriageReturnState),
    PotentialCharacterReference(PotentialCharacterReferenceState),
}

impl CharacterTransitionHandler for CharParserStateHandler {
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