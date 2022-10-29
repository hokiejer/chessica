use crate::reset::Reset;
use std::collections::HashMap;

pub fn perft(fen: &str, depth: u8) {
    use crate::utils::convert_bitstring_to_square;
    let mut total_move_count: u64 = 0;
    let mut r: Reset = crate::reset::new();
    let mut child: Reset = crate::reset::new();
    let fen = String::from(fen);
    r.init_from_fen(fen);

    while r.generate_next_move(&mut child) {
        let mut move_count: u64 = 0;
        child.in_place_move_tree(depth - 1, &mut move_count);
        total_move_count += move_count;
        let from_square = convert_bitstring_to_square(child.b_from);
        let to_square = convert_bitstring_to_square(child.b_to);
        println!("{}{} {}",from_square,to_square,move_count);
    }
    println!("");
    println!("{}",total_move_count);
}

pub fn count_possible_games(fen: &str, depth: u8) -> u64 {
    let mut move_count: u64 = 0;
    let mut r: Reset = crate::reset::new();
    let fen = String::from(fen);
    r.init_from_fen(fen);
    r.in_place_move_tree(depth, &mut move_count);
    move_count
}

pub fn burn() {

    //After valid_move work shift and improved ordering of Reset fields 1: 7m37.862s
    //After valid_move work shift and improved ordering of Reset fields 2: 7m41.491s
    //After valid_move work shift and improved ordering of Reset fields 3: 7m46.781s
 
    //After castle_bits and forced reset ordering 1: 7m49.908s
    //After castle_bits and forced reset ordering 2: 7m45.630s
    //After castle_bits and forced reset ordering 3: 7m46.438s
    
    //After King square tracking improvements 1: 7m58.442s
    //After King square tracking improvements 2: 7m58.142s

    //After Local Direct Check Updates 1: 8m56.675s
    //After Local Direct Check Updates 2: 8m59.084s

    //Before Local Direct Check Updates 1: 9m4.060s
    //Before Local Direct Check Updates 2: 9m0.868s

    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,7),3195901860,"starting position, ply=7");

    let fen = String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert_eq!(count_possible_games(&fen,6),8031647685,"Position 2 - Kiwipete, ply=6");

    // I need to figure out the answer here!!
    let fen = String::from("r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R w KQkq - 1 12");
    assert_eq!(count_possible_games(&fen,6),1547156972,"Morphy-Isouard 1858, ply=6");

}

impl Reset {


    pub fn in_place_move_tree(&mut self, depth: u8, move_count: &mut u64) {
        use crate::utils::hit_enter_to_continue;
        let mut search_flag: bool = true;
        if depth == 0 {
            *move_count += 1;
            //println!("LEAF {}",*move_count);
            //self.print_board_big();
            //println!("");
            //hit_enter_to_continue();
            return
        }
        let mut child = crate::reset::new();

        if depth == 1 {
            //println!("****************************PARENT****************************");
            //self.print_board_big();
            //println!("");
            //hit_enter_to_continue();
        }
        if search_flag {
            while self.generate_next_move(&mut child) {
                //let old: u64 = *move_count;
                child.in_place_move_tree(depth - 1, move_count);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    //use crate::reset;
    //use crate::utils;
    //use crate::reset::Reset;
}
