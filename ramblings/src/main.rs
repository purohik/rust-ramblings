#[path = "./inputs/io_processor.rs"]
mod io_processor;

fn main() {
  println!("Hello, world!");
  let _rev = io_processor::input_states("point");
  for (key, value) in _rev.into_iter() {
    println!("{}: {}", key, value.to_string());
  }
}
