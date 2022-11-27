#[macro_use]
extern crate lazy_static;
extern crate enum_map;

pub mod utils; //include "utils/mod.rs"
pub mod reset; //include "reset/mod.rs"
pub mod bitops; //include "bitops/mod.rs";
pub mod tree; //include "tree/mod.rs";
pub mod args; //include "args/mod.rs";

fn main() {
    use std::mem;
    use crate::reset::Reset;
    use crate::tree::Tree;
    use crate::args::profile::ProfileType;
    use crate::args::ArgStruct;
    use crate::args::process_args;
    use std::env;
    let args: Vec<String> = env::args().collect();

    let argdata: ArgStruct = process_args(args);
    //println!("Size of Reset: {}",mem::size_of::<Reset>());

    //crate::reset::profiling::perft("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1",6);
    if argdata.error() {
        return;
    }
    if argdata.profile_reset() {
        println!("Running profile script for resets...");
        use crate::reset::profiling;
        crate::reset::profiling::burn();

    } else if argdata.profile_tree() {
        println!("Running profile script for trees...");
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = tree::from_fen(starting_fen);
        let mut move_count: u64 = 0;
        t.simple_move_tree(5, &mut move_count);
        println!("Move count = {}",move_count);

    } else if argdata.profile_in_place_ab() {
        use crate::reset::r#const::SCORE_MIN;
        use crate::reset::r#const::SCORE_MAX;
        println!("Running profile script for In Place Alpha-Beta...");
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = tree::from_fen(starting_fen);
        let score = t.alpha_beta_in_place(8, SCORE_MAX, SCORE_MIN);
        println!("Score == {}",score);

    } else if argdata.profile_keep_depth_ab() {
        use crate::reset::r#const::SCORE_MIN;
        use crate::reset::r#const::SCORE_MAX;
        println!("Running profile script for Keep Depth Alpha-Beta...");
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = tree::from_fen(starting_fen);
        let score = t.alpha_beta_keep_depth(4, 8, SCORE_MAX, SCORE_MIN);
        println!("Score == {}",score);
    }

}
