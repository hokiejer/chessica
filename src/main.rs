#[macro_use]
extern crate lazy_static;

pub mod utils; //include "utils/mod.rs"
pub mod reset; //include "reset/mod.rs"
pub mod bitops; //include "bitops/mod.rs";
pub mod tree; //include "tree/mod.rs";

fn main() {
    use std::mem;
    use crate::reset::Reset;
    use crate::tree::Tree;

    let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("Size of Reset: {}",mem::size_of::<Reset>());
    let mut move_count: u64 = 0;
    let mut t: Tree = tree::from_fen(starting_fen);

    //t.simple_move_tree(5, &mut move_count);
    //println!("Move count = {}",move_count);

    use crate::reset::profiling;
    crate::reset::profiling::burn();

}
