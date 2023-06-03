#[path = "../types/letter_state.rs"]
mod letter_state;

pub struct CharState {
  letter: char,
  index: i32,
  state: letter_state::LetterState,
}

pub struct WordState {
  word_state: Vec<CharState>
}