#[macro_use]
extern crate lazy_static;
extern crate enum_map;

pub mod utils; //include "utils/mod.rs"
pub mod reset; //include "reset/mod.rs"
pub mod bitops; //include "bitops/mod.rs";
pub mod tree; //include "tree/mod.rs";
pub mod args; //include "args/mod.rs";
pub mod operator; //include "operator/mod.rs"
pub mod orchestrator; //include "operator/mod.rs"
pub mod cogitator; //include "operator/mod.rs"

fn main() {
    //use std::mem;
    use crate::tree::Tree;
    use crate::args::ArgStruct;
    use crate::args::process_args;
    use crate::args::usage;
    use std::env;
    use num_format::{Locale,ToFormattedString};
    let args: Vec<String> = env::args().collect();

    let argdata: ArgStruct = process_args(args);
    //println!("Size of Reset: {}",mem::size_of::<Reset>());

    //crate::reset::profiling::perft("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1",6);
    if argdata.error() {
        usage();
        return;
    }

    if argdata.profile {
        use crate::reset::r#const::SCORE_MIN;
        use crate::reset::r#const::SCORE_MAX;
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut t: Tree = tree::from_fen(starting_fen);
        let mut move_count: u64 = 0;

        if argdata.profile_reset() {
            println!("Running profile script for resets...");
            crate::reset::profiling::burn();

        } else if argdata.profile_tree() {
            println!("Running profile script for trees...");
            t.simple_move_tree(4, &mut move_count);
            println!("Move count = {}",move_count.to_formatted_string(&Locale::en));

        } else if argdata.profile_in_place_ab() {
            println!("Running profile script for In Place Alpha-Beta...");
            println!("Search Depth == {}",argdata.ab_search_depth);
            let score = t.alpha_beta_in_place(
                argdata.ab_search_depth, 
                SCORE_MAX, 
                SCORE_MIN, 
                &mut move_count
            );
            println!("Score == {}  Move count == {}",score,move_count.to_formatted_string(&Locale::en));

        } else if argdata.profile_keep_depth_ab() {
            println!("Running profile script for Keep Depth Alpha-Beta...");
            println!("Search Depth == {}, Keep Depth == {}",argdata.ab_search_depth,argdata.ab_keep_depth);
            let score = t.alpha_beta_keep_depth(
                4, 
                argdata.ab_search_depth, 
                SCORE_MAX, 
                SCORE_MIN, 
                &mut move_count
            );
            println!("Score == {}  Move count == {}",score,move_count.to_formatted_string(&Locale::en));

        } else if argdata.profile_iterative_keep_depth_ab() {
            println!("Running profile script for Iterative Keep Depth Alpha-Beta...");
            println!("Search Depth == {}, Keep Depth == {}",argdata.ab_search_depth,argdata.ab_keep_depth);
            let score = t.iterative_alpha_beta_keep_depth(
                argdata.ab_keep_depth, 
                argdata.ab_search_depth, 
                SCORE_MAX, 
                SCORE_MIN
            );
            println!("Score == {}",score);
        } else if argdata.profile_iterative_keep_depth_ab_promote() {
            println!("Running profile script for Iterative Keep Depth Alpha-Beta with Swap...");
            println!("Search Depth == {}, Keep Depth == {}",argdata.ab_search_depth,argdata.ab_keep_depth);
            let score = t.iterative_alpha_beta_keep_depth_promote(argdata.ab_keep_depth, argdata.ab_search_depth, SCORE_MAX, SCORE_MIN);
            println!("Score == {}",score);
        }
    } else {
        let mut operator = operator::new();
        operator.launch();
        // Game Time!
        // Build game object
        // Spawn Orchestrator thread
        // Launch Operator method
    }

}
