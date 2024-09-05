use std::marker::PhantomData;
use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::line_ending::LineEnding;
use crate::parser::transition::{Transition, TransitionEffect};

pub struct Handler<SuperState, State> {
    state: State,
    _super: PhantomData<SuperState>,
}

impl<SuperState, State> Handler<SuperState, State>
where
    SuperState: From<Self>,
{
    pub fn new(state: State) -> Self {
        Self {
            state,
            _super: Default::default(),
        }
    }

    pub fn state(self) -> State {
        self.state
    }

    fn handle_effect<F>(mut self, effect: F) ->
    NonDeterministicTransitionEffect<SuperState, Option<Block>>
    where
        F: FnOnce(State) -> NonDeterministicTransitionEffect<State, Option<Block>>,
    {
        match effect(self.state).content() {
            (Some(state), block) => {
                self.state = state;
                NonDeterministicTransitionEffect::new(
                    self.into(),
                    block,
                )
            }
            (_, Some(block)) => {
                NonDeterministicTransitionEffect::complete::<Block>(block)
            }
            _ => {
                NonDeterministicTransitionEffect::dismiss()
            }
        }
    }
}

impl<SuperState, State> Transition for Handler<SuperState, State>
where
    State: Transition<Effect=NonDeterministicTransitionEffect<State, Option<Block>>>,
    SuperState: From<Self>,
{
    type Effect = NonDeterministicTransitionEffect<SuperState, Option<Block>>;

    fn transition(self, character: Character) -> Self::Effect {
        self.handle_effect(|state| state.transition(character))
    }

    fn end_line(self, line_ending: LineEnding) -> Self::Effect {
        self.handle_effect(|state| state.end_line(line_ending))
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        self.state.end()
    }
}