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

    let mut t: Tree = tree::from_fen(starting_fen);

    let mut i = 0;
    while t.add_next_child() {
        let mut child = &mut t.children[i];
        child.print();
        i += 1;
    }
}
