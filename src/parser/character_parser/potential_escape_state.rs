use crate::parser::character::Character;
use crate::parser::character_parser::{CharacterParserState, TransitHandler, StateTransition, SubStateTransit, CharacterParserStateHandler};
use crate::parser::character_parser::character_transition_result::{CharacterTransitionResult, PositionedLineEnding};
use crate::parser::character_parser::default_state::DefaultState;
use crate::unicode;

#[derive(Default)]
pub struct PotentialEscapeState;

impl SubStateTransit for PotentialEscapeState {
    type Handler = CharacterParserStateHandler;

    fn transition(&self, character: char) -> StateTransition<<Self::Handler as TransitHandler>::State, <Self::Handler as TransitHandler>::TransitionResult> {
        if let Ok(character) = Character::new_unescaped(character) {
            StateTransition::transition_into::<DefaultState>(
                CharacterTransitionResult::characters(
                    vec![
                        character,
                    ]
                ),
            )
        } else {
            let transition = DefaultState.transition(character);
            let mut result = CharacterTransitionResult::characters(
                vec![
                    vec![
                        Character::Unescaped(unicode::BACKSLASH),
                    ],
                    transition.result.characters,
                ].concat()
            );

            if let Some(
                PositionedLineEnding::Before(line_ending) |
                PositionedLineEnding::After(line_ending)
            ) = transition.result.line_ending {
                result = result.with_line_ending(PositionedLineEnding::After(line_ending));
            }

            StateTransition::new(
                transition.state,
                result,
            )
        }
    }

    fn end(&self) -> <Self::Handler as TransitHandler>::TransitionResult {
        CharacterTransitionResult::characters(
            vec![
                Character::Unescaped(unicode::BACKSLASH)
            ]
        )
    }
}

impl From<PotentialEscapeState> for CharacterParserState {
    fn from(value: PotentialEscapeState) -> Self {
        CharacterParserState::PotentialEscape(value)
    }
}