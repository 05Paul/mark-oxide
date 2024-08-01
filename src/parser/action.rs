use crate::parser::document::block::Block;
use crate::parser::state::State;

#[derive(Clone)]
pub enum Action {
    Pass(State),
    Complete(Block),
    Bi {
        first: Box<Action>,
        second: Box<Action>,
    },
    Dismiss,
}

impl Action {
    pub fn merge(self, action: Action) -> Action {
        match (self, action) {
            (Action::Dismiss, action) | (action, Action::Dismiss)=> action,
            (first, second) => {
                Action::Bi {
                    first: Box::new(first),
                    second: Box::new(second),
                }
            } ,
        }
    }
}