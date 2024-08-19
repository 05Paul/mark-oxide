use crate::parser::character::Character;
use crate::parser::character_parser::{CharacterParserState, CharParserStateHandler};
use crate::parser::character_parser::character_transition::{CharacterStateTransition, CharacterSubTransition, CharacterTransitionHandler, CharacterTransitionResult, PositionedLineEnding};
use crate::parser::character_parser::default::DefaultState;
use crate::unicode;

#[derive(Default)]
pub struct PotentialEscapeState;

impl CharacterSubTransition for PotentialEscapeState {
    type Handler = CharParserStateHandler;

    fn transition(&self, character: char) -> CharacterStateTransition<<Self::Handler as CharacterTransitionHandler>::State, <Self::Handler as CharacterTransitionHandler>::TransitionResult> {
        if let Ok(character) = Character::new_unescaped(character) {
            CharacterStateTransition::transition_into::<DefaultState>(
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

            CharacterStateTransition::new(
                transition.state,
                result,
            )
        }
    }

    fn end(&self) -> <Self::Handler as CharacterTransitionHandler>::TransitionResult {
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