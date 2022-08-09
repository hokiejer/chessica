use crate::reset::Reset;
use crate::reset::r#const::B_NOT_TOP_EDGE;
use crate::reset::r#const::B_NOT_UL_EDGE;
use crate::reset::r#const::B_NOT_UR_EDGE;
use crate::reset::r#const::B_NOT_DL_EDGE;
use crate::reset::r#const::B_NOT_DR_EDGE;
use crate::reset::r#const::B_RANK_1;
use crate::reset::r#const::B_RANK_2;
use crate::reset::r#const::B_RANK_7;
use crate::reset::r#const::B_RANK_8;

impl Reset {

    pub fn generate_next_white_pawn_move(&mut self, child: &mut Reset) -> bool {
        // Forward one (not promotion)
        if self.move_id < 20 {
            if self.b_current_piece & B_NOT_TOP_EDGE != 0 {
                let b_destination = self.b_current_piece << 8;
                if (b_destination & self.b_all == 0) && 
                    self.add_move_if_valid(child, b_destination) 
                {
                    if b_destination & B_RANK_8 == 0 {
                        self.move_id = 20;
                    } else {
                        child.promotion = 1;
                        child.b_pawns &= !b_destination;
                        match self.move_id % 10 {
                            0 => { // Promote to knight
                                child.b_knights |= b_destination;
                                child.material += 2;
                                self.move_id = 11;
                            },
                            1 => { // Promote to bishop
                                child.b_bishops |= b_destination;
                                child.material += 2;
                                self.move_id = 12;
                            },
                            2 => { // Promote to rook
                                child.b_rooks |= b_destination;
                                child.material += 4;
                                self.move_id = 13;
                            },
                            3 => { // Promote to queen
                                child.b_queens |= b_destination;
                                child.material += 8;
                                self.move_id = 20;
                            },
                            _ => panic!("Shouldn't get here!"),
                        }
                        if !child.black_is_safe(child.b_kings & child.b_black) {
                            child.in_check = 1;
                        }
                    }
                    return true;
                }
            }
        }
                
        // Forward two
        if self.move_id < 30 {
            self.move_id = 30;
            let b_one_square = self.b_current_piece << 8;
            let b_destination = self.b_current_piece << 16; // This won't work as coded

            if (self.b_current_piece & B_RANK_2 != 0) &&
                ((b_one_square & self.b_all) == 0) &&
                ((b_destination & self.b_all) == 0) &&
                self.add_move_if_valid(child, b_destination)
            {
                //Don't forget to set the EP square!
                return true;
            }
        }

        // Capture Left
        if self.move_id < 40 {
            let b_destination = self.b_current_piece << 9;
            if (self.b_current_piece & B_NOT_UL_EDGE != 0) && 
                (b_destination & self.b_black != 0) && 
                self.add_move_if_valid(child, b_destination) 
            {
                if b_destination & B_RANK_8 == 0 {
                    self.move_id = 40;
                } else {
                    child.promotion = 1;
                    child.b_pawns &= !b_destination;
                    match self.move_id % 10 {
                        0 => { // Promote to knight
                            child.b_knights |= b_destination;
                            child.material += 2;
                            self.move_id = 31;
                        },
                        1 => { // Promote to bishop
                            child.b_bishops |= b_destination;
                            child.material += 2;
                            self.move_id = 32;
                        },
                        2 => { // Promote to rook
                            child.b_rooks |= b_destination;
                            child.material += 4;
                            self.move_id = 33;
                        },
                        3 => { // Promote to queen
                            child.b_queens |= b_destination;
                            child.material += 8;
                            self.move_id = 40;
                        },
                        _ => panic!("Shouldn't get here!"),
                    }
                    if !child.black_is_safe(child.b_kings & child.b_black) {
                        child.in_check = 1;
                    }
                }
                return true;
            }
        }

        // Capture Right
        if self.move_id < 50 {
            let b_destination = self.b_current_piece << 7;
            if (self.b_current_piece & B_NOT_UR_EDGE != 0) && 
                (b_destination & self.b_black != 0) && 
                self.add_move_if_valid(child, b_destination) 
            {
                if b_destination & B_RANK_8 == 0 {
                    self.move_id = 50;
                } else {
                    child.promotion = 1;
                    child.b_pawns &= !b_destination;
                    match self.move_id % 10 {
                        0 => { // Promote to knight
                            child.b_knights |= b_destination;
                            child.material += 2;
                            self.move_id = 41;
                        },
                        1 => { // Promote to bishop
                            child.b_bishops |= b_destination;
                            child.material += 2;
                            self.move_id = 42;
                        },
                        2 => { // Promote to rook
                            child.b_rooks |= b_destination;
                            child.material += 4;
                            self.move_id = 43;
                        },
                        3 => { // Promote to queen
                            child.b_queens |= b_destination;
                            child.material += 8;
                            self.move_id = 50;
                        },
                        _ => panic!("Shouldn't get here!"),
                    }
                    if !child.black_is_safe(child.b_kings & child.b_black) {
                        child.in_check = 1;
                    }
                }
                return true;
            }
        }

        self.consider_next_moveable_piece();
        false
    }

    pub fn generate_next_black_pawn_move(&mut self, child: &mut Reset) -> bool {
        true
    }

    pub fn generate_next_pawn_move(&mut self, child: &mut Reset) -> bool {
        if self.white_to_move() {
            self.generate_next_white_pawn_move(child)
        } else { 
            self.generate_next_black_pawn_move(child)
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r.initialize_move_generation();
        r
    }

    #[test]
    fn pawn_moves_white_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("h2".to_string());

        // h2 to h3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h2".to_string()));
        assert_eq!(r.move_id,20);

        // h2 to h4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g2".to_string()));
        assert_eq!(r.move_id,10);

        // g2 to g3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g2".to_string()));
        assert_eq!(r.move_id,20);

        // g2 to g4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,10);

        // f2 to f3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,20);

        // f2 to f4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e2".to_string()));
        assert_eq!(r.move_id,10);

        // e2 to e3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e2".to_string()));
        assert_eq!(r.move_id,20);

        // e2 to e4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,10);

        // d2 to d3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,20);

        // d2 to d4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c2".to_string()));
        assert_eq!(r.move_id,10);

        // c2 to c3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c2".to_string()));
        assert_eq!(r.move_id,20);

        // c2 to c4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,10);

        // b2 to b3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,20);

        // b2 to b4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,10);

        // a2 to a3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,20);

        // a2 to a4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(r.move_id,10);

    }

    #[test]
    fn pawn_moves_white_captures() {
        let mut r = prep_board("k7/8/2p2p2/3PP3/p4p1p/1P4P1/P1P5/K7 w - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("g3".to_string());

        // g3 to g4
        let fen = String::from("k7/8/2p2p2/3PP3/p4pPp/1P6/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g3".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // g3 to f4
        let fen = String::from("k7/8/2p2p2/3PP3/p4P1p/1P6/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g3".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // g3 to h4
        let fen = String::from("k7/8/2p2p2/3PP3/p4p1P/1P6/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g3".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b3".to_string()));
        assert_eq!(r.move_id,10);

        // b3 to b4
        let fen = String::from("k7/8/2p2p2/3PP3/pP3p1p/6P1/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b3".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // b3 to a4
        let fen = String::from("k7/8/2p2p2/3PP3/P4p1p/6P1/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b3".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e5".to_string()));
        assert_eq!(r.move_id,10);

        // e5 to e6
        let fen = String::from("k7/8/2p1Pp2/3P4/p4p1p/1P4P1/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e5".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e5 to f5
        let fen = String::from("k7/8/2p2P2/3P4/p4p1p/1P4P1/P1P5/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e5".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d5".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn pawn_moves_white_promotion() {
        let mut r = prep_board("k2r4/2P1P2P/8/8/8/8/8/K7 w - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("h7".to_string());

        // h7 to h8 Knight
        let fen = String::from("k2r3N/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // h7 to h8 Bishop
        let fen = String::from("k2r3B/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // h7 to h8 Rook
        let fen = String::from("k2r3R/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);

        // h7 to h8 Queen
        let fen = String::from("k2r3Q/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,10);

        // e7 to e8 Knight
        let fen = String::from("k2rN3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // e7 to e8 Bishop
        let fen = String::from("k2rB3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // e7 to e8 Rook
        let fen = String::from("k2rR3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);

        // e7 to e8 Queen
        let fen = String::from("k2rQ3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e7 to d8 Knight
        let fen = String::from("k2N4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,1);

        // e7 to d8 Bishop
        let fen = String::from("k2B4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,1);

        // e7 to d8 Rook
        let fen = String::from("k2R4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,33);
        assert_eq!(child.capture,1);

        // e7 to d8 Queen
        let fen = String::from("k2Q4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,10);

        // c7 to c8 Knight
        let fen = String::from("k1Nr4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // c7 to c8 Bishop
        let fen = String::from("k1Br4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // c7 to c8 Rook
        let fen = String::from("k1Rr4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);

        // c7 to c8 Queen
        let fen = String::from("k1Qr4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // c7 to d8 Knight
        let fen = String::from("k2N4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,1);

        // c7 to d8 Bishop
        let fen = String::from("k2B4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,1);

        // c7 to d8 Rook
        let fen = String::from("k2R4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,43);
        assert_eq!(child.capture,1);

        // c7 to d8 Queen
        let fen = String::from("k2Q4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(r.move_id,10);

    }
}
