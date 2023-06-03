use std::collections::HashMap;
use std::io;

pub fn input_states(word: String) -> HashMap<char, LetterState> {
  let input_state = Hashmap::new();

  for letter in word {
    let mut state_position;
    print!("Enter status of {}:", letter);
    io::stdin().read_line(state_position)
      .expect("Please enter a number in the range [1:3] :/");

    input_state.insert(letter, state_position.parse::<i32>().unwrap().into());
  }
}