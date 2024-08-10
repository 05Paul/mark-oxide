use crate::parser::character_parser::{CharacterParserState, TransitHandler, StateTransition, SubStateTransit, CharacterParserStateHandler};

pub struct PotentialCharacterReferenceState {
    content: String,
}

impl SubStateTransit for PotentialCharacterReferenceState {
    type Handler = CharacterParserStateHandler;

    fn transition(&self, character: char) -> StateTransition<<Self::Handler as TransitHandler>::State, <Self::Handler as TransitHandler>::TransitionResult> {
        todo!()
    }

    fn end(&self) -> <Self::Handler as TransitHandler>::TransitionResult {
        todo!()
    }
}

impl From<char> for PotentialCharacterReferenceState {
    fn from(value: char) -> Self {
        Self {
            content: value.to_string(),
        }
    }
}

impl From<PotentialCharacterReferenceState> for CharacterParserState {
    fn from(value: PotentialCharacterReferenceState) -> Self {
        CharacterParserState::PotentialCharacterReference(value)
    }
}