use std::collections::HashMap;
use std::io;
use std::io::Write;

#[path = "../types/letter_state.rs"]
mod letter_state;
#[path = "../types/word_state.rs"]
mod word;

pub fn input_states(word: &str) -> HashMap<char, letter_state::LetterState> {
  let mut input_state = HashMap::new();

  for letter in word.chars() {
    let state_position = &mut "".to_string();
    print!("Enter status of {}:", letter);
    io::stdout().flush().unwrap();
    io::stdin().read_line(state_position)
      .expect("Please enter a number in the range [1:3] :/");

    input_state.insert(letter, state_position.trim().parse::<i32>().unwrap().into());
  }
  input_state
}

// TODO: move the above implementation from HashMap to Vector-based structure to maintain the order of chars so that it's easy to compare.
pub fn input_states_v2(word: &str) -> WordState {
  let mut res = word::WordState();
  for letter in word.chars() {
    print!("Enter the status of {}", letter);
    io::stdout().flush().unwrap();

    let state_position = &mut "".to_string();
    io::stdin().read_line(state_position)
      .expect("Please enter a number in the range [1:3] :/");
  }
  res
}