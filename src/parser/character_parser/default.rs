use crate::parser::character::{Character, InvalidCharacterCategoryError};
use crate::parser::character_parser::{CharacterParserState, CharParserStateHandler, PotentialCharacterReferenceState};
use crate::parser::character_parser::character_transition::{CharacterSubTransition, CharacterTransitionHandler, CharacterTransitionResult, CharacterStateTransition};
use crate::parser::character_parser::potential_escape::PotentialEscapeState;
use crate::parser::line_ending::LineEnding;

#[derive(Default)]
pub struct DefaultState;

impl CharacterSubTransition for DefaultState {
    type Handler = CharParserStateHandler;

    fn transition(&self, character: char) -> CharacterStateTransition<<Self::Handler as CharacterTransitionHandler>::State, <Self::Handler as CharacterTransitionHandler>::TransitionResult> {
        match Character::try_from(character) {
            Ok(character) => CharacterStateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::characters(
                    vec![
                        character
                    ]
                ),
            ),
            Err(InvalidCharacterCategoryError::Escape) => CharacterStateTransition::transition_into::<PotentialEscapeState>(
                CharacterTransitionResult::default(),
            ),
            Err(InvalidCharacterCategoryError::LineEnding(LineEnding::LineFeed)) => CharacterStateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::line_ending_before(LineEnding::LineFeed),
            ),
            Err(InvalidCharacterCategoryError::LineEnding(_)) => CharacterStateTransition::transition_into::<CharacterParserState>(
                CharacterTransitionResult::default(),
            ),
            Err(InvalidCharacterCategoryError::Reference) => CharacterStateTransition::transition_into_from::<PotentialCharacterReferenceState>(
                character,
                CharacterTransitionResult::default(),
            )
        }
    }

    fn end(&self) -> <Self::Handler as CharacterTransitionHandler>::TransitionResult {
        CharacterTransitionResult::default()
    }
}

impl From<DefaultState> for CharacterParserState {
    fn from(value: DefaultState) -> Self {
        CharacterParserState::Default(value)
    }
}