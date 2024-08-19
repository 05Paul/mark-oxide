use crate::parser::character_parser::{CharacterParserState, CharParserStateHandler};
use crate::parser::character_parser::character_transition::{CharacterSubTransition, CharacterTransitionHandler, CharacterStateTransition};

pub struct PotentialCharacterReferenceState {
    content: String,
}

impl CharacterSubTransition for PotentialCharacterReferenceState {
    type Handler = CharParserStateHandler;

    fn transition(&self, character: char) -> CharacterStateTransition<<Self::Handler as CharacterTransitionHandler>::State, <Self::Handler as CharacterTransitionHandler>::TransitionResult> {
        todo!()
    }

    fn end(&self) -> <Self::Handler as CharacterTransitionHandler>::TransitionResult {
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