mod utils; //include "utils/mod.rs"
mod reset; //include "reset/mod.rs"
mod bitops; //include "bitops/mod.rs";

fn main() {
      let mut r = reset::new();
      let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
      r.init_from_fen(starting_fen);
      use std::mem;
      use crate::reset::Reset;
      println!("Size of Reset: {}",mem::size_of::<Reset>());
      r.print();
}
