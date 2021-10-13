mod reset; //include "reset/mod.rs"

fn main() {
      let r = reset::new();
      let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
      let s = reset::from_fen(starting_fen);
      println!("Hello world!");
      reset::sub1::hello();
      r.print();
}
