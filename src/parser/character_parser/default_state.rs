use crate::parser::character::{Character, InvalidCharacterCategoryError};
use crate::parser::character_parser::{CharacterParserState, PotentialCharacterReferenceState, TransitHandler, StateTransition, SubStateTransit, CharacterParserStateHandler};
use crate::parser::character_parser::character_transition_result::CharacterTransitionResult;
use crate::parser::character_parser::potential_escape_state::PotentialEscapeState;
use crate::parser::state::LineEnding;

#[derive(Default)]
pub struct DefaultState;

impl SubStateTransit for DefaultState {
    type Handler = CharacterParserStateHandler;

    fn transition(&self, character: char) -> StateTransition<<Self::Handler as TransitHandler>::State, <Self::Handler as TransitHandler>::TransitionResult> {
        match Character::try_from(character) {
            Ok(character) => StateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::characters(
                    vec![
                        character
                    ]
                ),
            ),
            Err(InvalidCharacterCategoryError::Escape) => StateTransition::transition_into::<PotentialEscapeState>(
                CharacterTransitionResult::default(),
            ),
            Err(InvalidCharacterCategoryError::LineEnding(LineEnding::LineFeed)) => StateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::line_ending_before(LineEnding::LineFeed),
            ),
            Err(InvalidCharacterCategoryError::LineEnding(_)) => StateTransition::transition_into::<CharacterParserState>(
                CharacterTransitionResult::default(),
            ),
            Err(InvalidCharacterCategoryError::Reference) => StateTransition::transition_into_from::<PotentialCharacterReferenceState>(
                character,
                CharacterTransitionResult::default(),
            )
        }
    }

    fn end(&self) -> <Self::Handler as TransitHandler>::TransitionResult {
        CharacterTransitionResult::default()
    }
}

impl From<DefaultState> for CharacterParserState {
    fn from(value: DefaultState) -> Self {
        CharacterParserState::Default(value)
    }
}