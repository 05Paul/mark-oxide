use crate::parser::character_parser::{CharacterParserState, TransitHandler, StateTransition, SubStateTransit, CharacterParserStateHandler};
use crate::parser::character_parser::character_transition_result::{CharacterTransitionResult, PositionedLineEnding};
use crate::parser::character_parser::default_state::DefaultState;
use crate::parser::state::LineEnding;
use crate::unicode;

#[derive(Default)]
pub struct CarriageReturnState;

impl SubStateTransit for CarriageReturnState {
    type Handler = CharacterParserStateHandler;

    fn transition(&self, character: char) -> StateTransition<<Self::Handler as TransitHandler>::State, <Self::Handler as TransitHandler>::TransitionResult> {
        match character {
            unicode::CARRIAGE_RETURN => StateTransition::transition_into::<CarriageReturnState>(
                CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturn),
            ),
            unicode::LINE_FEED => StateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturnLineFeed),
            ),
            character => {
                let next = DefaultState.transition(character);
                StateTransition::new(
                    next.state,
                    CharacterTransitionResult::characters(next.result.characters)
                        .with_line_ending(PositionedLineEnding::Before(LineEnding::CarriageReturn)),
                )
            }
        }
    }

    fn end(&self) -> <Self::Handler as TransitHandler>::TransitionResult {
        CharacterTransitionResult::line_ending_before(LineEnding::CarriageReturn)
    }
}

impl From<CarriageReturnState> for CharacterParserState {
    fn from(value: CarriageReturnState) -> Self {
        CharacterParserState::CarriageReturn(value)
    }
}