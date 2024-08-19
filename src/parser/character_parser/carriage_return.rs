use crate::parser::character_parser::{CharacterParserState, CharParserStateHandler};
use crate::parser::character_parser::character_transition::{CharacterSubTransition, CharacterTransitionHandler, CharacterTransitionResult, PositionedLineEnding, CharacterStateTransition};
use crate::parser::character_parser::default::DefaultState;
use crate::parser::line_ending::LineEnding;
use crate::unicode;

#[derive(Default)]
pub struct CarriageReturnState;

impl CharacterSubTransition for CarriageReturnState {
    type Handler = CharParserStateHandler;

    fn transition(&self, character: char) -> CharacterStateTransition<<Self::Handler as CharacterTransitionHandler>::State, <Self::Handler as CharacterTransitionHandler>::TransitionResult> {
        match character {
            unicode::CARRIAGE_RETURN => CharacterStateTransition::transition_into::<CarriageReturnState>(
                CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturn),
            ),
            unicode::LINE_FEED => CharacterStateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturnLineFeed),
            ),
            character => {
                let next = DefaultState.transition(character);
                CharacterStateTransition::new(
                    next.state,
                    CharacterTransitionResult::characters(next.result.characters)
                        .with_line_ending(PositionedLineEnding::Before(LineEnding::CarriageReturn)),
                )
            }
        }
    }

    fn end(&self) -> <Self::Handler as CharacterTransitionHandler>::TransitionResult {
        CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturn)
    }
}

impl From<CarriageReturnState> for CharacterParserState {
    fn from(value: CarriageReturnState) -> Self {
        CharacterParserState::CarriageReturn(value)
    }
}