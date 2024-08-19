use crate::parser::character::Character;
use crate::parser::document::block::Block;
use crate::parser::document::leaf::Leaf;
use crate::parser::effect::NonDeterministicTransitionEffect;
use crate::parser::state::{LineEnding, State};
use crate::parser::state::sub_state::SubState;
use crate::parser::transition::{Transition, TransitionEffect};
use crate::unicode;

#[derive(Clone)]
pub struct IndentedCodeBlockState {
    text: String,
    line_break_buffer: String,
    leading_space_count: usize,
    leading_spaces: bool,
    newline: bool,
}

impl IndentedCodeBlockState {
    pub fn new(leading_internal_spaces: usize) -> Self {
        Self {
            text: "".repeat(leading_internal_spaces).into(),
            line_break_buffer: "".into(),
            leading_space_count: 4,
            leading_spaces: false,
            newline: true,
        }
    }
}


impl Transition for IndentedCodeBlockState {
    type Effect = NonDeterministicTransitionEffect<SubState, Option<Block>>;

    fn transition(mut self, character: Character) -> Self::Effect {
        match (character.clone(), self.newline, self.leading_spaces, self.leading_space_count) {
            (Character::Unescaped(unicode::SPACE | unicode::TAB), _, _, 0..=3) => {
                self.leading_space_count += character.space_count();
                self.newline = false;

                NonDeterministicTransitionEffect::pass(self)
            }
            (Character::Unescaped(unicode::SPACE | unicode::TAB), _, true, 4..) => {
                self.line_break_buffer.push_str(&*character.to_string());
                self.leading_space_count += character.space_count();

                NonDeterministicTransitionEffect::pass(self)
            }
            (character, _, _, 4..) => {
                self.text.push_str(&*self.line_break_buffer);
                self.text.push_str(&*character.to_raw_string());
                self.newline = false;
                self.line_break_buffer = "".into();

                NonDeterministicTransitionEffect::pass(self)
            }
            (character, _, _, 0..=3) => {
                let (state, _) = State::default()
                    .transition(character).content();
                NonDeterministicTransitionEffect::new(
                    SubState::IntoSuper(state),
                    Some(Leaf::IndentedCodeBlock {
                        text: self.text.trim_end().to_string()
                    }.into()),
                )
            }
        }
    }

    fn end_line(mut self, line_ending: LineEnding) -> Self::Effect {
        self.leading_spaces = true;
        self.newline = true;
        self.leading_space_count = 0;

        if self.text.trim().is_empty() {
            self.text = "".into();
            self.line_break_buffer = "".into();
            self.leading_space_count = 0;

            return NonDeterministicTransitionEffect::pass(self);
        }


        if self.leading_spaces {
            self.line_break_buffer.push_str(&*line_ending.to_string());

            return NonDeterministicTransitionEffect::pass(self);
        }

        self.text.push_str(&*line_ending.to_string());
        self.line_break_buffer = "".into();

        NonDeterministicTransitionEffect::pass(self)
    }

    fn end(self) -> <Self::Effect as TransitionEffect>::Outcome {
        Some(
            Leaf::IndentedCodeBlock {
                text: self.text.trim_end_matches([
                    unicode::CARRIAGE_RETURN,
                    unicode::LINE_FEED,
                ]).to_string()
            }.into()
        )
    }
}

impl From<IndentedCodeBlockState> for SubState {
    fn from(value: IndentedCodeBlockState) -> Self {
        SubState::IndentedCodeBlock(value)
    }
}