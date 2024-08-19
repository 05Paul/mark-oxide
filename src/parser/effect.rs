use crate::parser::character::Character;
use crate::parser::line_ending::LineEnding;
use crate::parser::transition::{Transition, TransitionEffect};

pub struct DeterministicTransitionEffect<State, Outcome> {
    state: State,
    outcome: Outcome,
}

impl<State, Outcome> DeterministicTransitionEffect<State, Outcome> {
    pub fn new(state: State, outcome: Outcome) -> Self {
        Self {
            state,
            outcome,
        }
    }
}

impl<State, Outcome> DeterministicTransitionEffect<State, Outcome>
where
    State: Default,
{
    pub fn complete(outcome: impl Into<Outcome>) -> Self {
        Self {
            state: State::default(),
            outcome: outcome.into(),
        }
    }
}

impl<State, Outcome> DeterministicTransitionEffect<State, Outcome>
where
    Outcome: Default,
{
    pub fn pass(state: impl Into<State>) -> Self {
        Self {
            state: state.into(),
            outcome: Outcome::default(),
        }
    }

    pub fn transition_into<T: Into<State>>(value: impl Into<T>) -> Self {
        Self {
            state: value.into().into(),
            outcome: Outcome::default(),
        }
    }
}

impl<State, Outcome> DeterministicTransitionEffect<State, Outcome>
where
    State: Default,
    Outcome: Default,
{
    pub fn dismiss() -> Self {
        Self {
            state: State::default(),
            outcome: Outcome::default(),
        }
    }
}

impl<State, Outcome> TransitionEffect for DeterministicTransitionEffect<State, Outcome>
where
    State: Transition,
{
    type State = State;
    type Outcome = Outcome;

    fn content(self) -> (Self::State, Self::Outcome) {
        (self.state, self.outcome)
    }
}

pub struct NonDeterministicTransitionEffect<State, Outcome> {
    state: Option<State>,
    outcome: Outcome,
}

impl<State, Outcome> NonDeterministicTransitionEffect<State, Outcome> {
    pub fn new(state: State, outcome: Outcome) -> Self {
        Self {
            state: Some(state),
            outcome,
        }
    }

    pub fn complete<T: Into<Outcome>>(outcome: impl Into<T>) -> Self {
        Self {
            state: Default::default(),
            outcome: outcome.into().into(),
        }
    }
}

impl<State, Outcome> NonDeterministicTransitionEffect<State, Outcome>
where
    Outcome: Default,
{
    pub fn pass(state: impl Into<State>) -> Self {
        Self {
            state: Some(state.into()),
            outcome: Outcome::default(),
        }
    }

    pub fn transition_into<T: Into<State>>(state: impl Into<T>) -> Self {
        Self {
            state: Some(state.into().into()),
            outcome: Outcome::default(),
        }
    }

    pub fn dismiss() -> Self {
        Self {
            state: Default::default(),
            outcome: Outcome::default(),
        }
    }
}

impl<State, Outcome> TransitionEffect for NonDeterministicTransitionEffect<State, Outcome> {
    type State = Option<State>;
    type Outcome = Outcome;

    fn content(self) -> (Self::State, Self::Outcome) {
        (self.state, self.outcome)
    }
}

impl Transition for () {
    type Effect = DeterministicTransitionEffect<(), ()>;

    fn transition(self, _: Character) -> Self::Effect {
        DeterministicTransitionEffect::dismiss()
    }

    fn end_line(self, _: LineEnding) -> Self::Effect {
        DeterministicTransitionEffect::dismiss()
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        Default::default()
    }
}