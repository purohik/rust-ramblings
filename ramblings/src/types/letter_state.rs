use std::fmt;

pub enum LetterState {
  CorrectPosition,
  IncorrectPosition,
  NotPresent,
}

impl From<i32> for LetterState {
  fn from(state: i32) -> LetterState {
    match state {
      1 => LetterState::CorrectPosition,
      2 => LetterState::IncorrectPosition,
      3 => LetterState::NotPresent,
      _ => panic!("Not a valid state for the letter :/")
    }
  }
}

impl fmt::Display for LetterState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LetterState::CorrectPosition => write!(f, "Correct!"),
      LetterState::IncorrectPosition => write!(f, "Incorrect!"),
      LetterState::NotPresent => write!(f, "Not Present!"),
    }
  }
}