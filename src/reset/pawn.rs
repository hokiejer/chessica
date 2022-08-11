use crate::reset::Reset;
use crate::reset::r#const::B_NOT_TOP_EDGE;
use crate::reset::r#const::B_NOT_BOTTOM_EDGE;
use crate::reset::r#const::B_NOT_UL_EDGE;
use crate::reset::r#const::B_NOT_UR_EDGE;
use crate::reset::r#const::B_NOT_DL_EDGE;
use crate::reset::r#const::B_NOT_DR_EDGE;
use crate::reset::r#const::B_RANK_1;
use crate::reset::r#const::B_RANK_2;
use crate::reset::r#const::B_RANK_7;
use crate::reset::r#const::B_RANK_8;

impl Reset {

    pub fn generate_promotion_moves(&mut self, child: &mut Reset, move_base: u8) {
        child.promotion = 1;
        child.b_pawns &= !child.b_to;
        let multiplier = if self.white_to_move() {
            1
        } else {
            -1
        };
        match self.move_id % 10 {
            0 => { // Promote to knight
                child.b_knights |= child.b_to;
                child.material += 2 * multiplier;
                self.move_id = move_base + 1;
            },
            1 => { // Promote to bishop
                child.b_bishops |= child.b_to;
                child.material += 2 * multiplier;
                self.move_id = move_base + 2;
            },
            2 => { // Promote to rook
                child.b_rooks |= child.b_to;
                child.material += 4 * multiplier;
                self.move_id = move_base + 3;
            },
            3 => { // Promote to queen
                child.b_queens |= child.b_to;
                child.material += 8 * multiplier;
                self.move_id = move_base + 10;
            },
            _ => panic!("Shouldn't get here!"),
        }
        if self.white_to_move() {
            if !child.black_is_safe(child.b_kings & child.b_black) {
                child.in_check = 1;
            }
        } else {
            if !child.white_is_safe(child.b_kings & child.b_white) {
                child.in_check = 1;
            }
        }
    }

    pub fn generate_next_pawn_move(&mut self, child: &mut Reset) -> bool {
        let mut b_destination: u64;
        let mut b_not_last_rank: u64;
        let b_forward_one: u64;
        let b_starting_rank: u64;
        let b_can_capture_left: u64;
        let b_can_capture_right: u64;
        let mut b_opponent: u64;

        // Forward one (not promotion)
        if self.move_id < 20 {
            if self.white_to_move() {
                b_destination = self.b_current_piece << 8;
                b_not_last_rank = B_NOT_TOP_EDGE;
            } else {
                b_destination = self.b_current_piece >> 8;
                b_not_last_rank = B_NOT_BOTTOM_EDGE;
            }
            if self.b_current_piece & b_not_last_rank != 0 {
                if (b_destination & self.b_all == 0) && 
                    self.add_move_if_valid(child, b_destination) 
                {
                    if b_destination & b_not_last_rank != 0 {
                        self.move_id = 20;
                    } else {
                        self.generate_promotion_moves(child, 10);
                    }
                    return true;
                }
            }
        }
                
        // Forward two
        if self.move_id < 30 {
            self.move_id = 30;
            if self.white_to_move() {
                b_forward_one = self.b_current_piece << 8;
                b_destination = self.b_current_piece << 16;
                b_starting_rank = B_RANK_2;
            } else {
                b_forward_one = self.b_current_piece >> 8;
                b_destination = self.b_current_piece >> 16;
                b_starting_rank = B_RANK_7;
            }

            if (self.b_current_piece & b_starting_rank != 0) &&
                ((b_forward_one & self.b_all) == 0) &&
                ((b_destination & self.b_all) == 0) &&
                self.add_move_if_valid(child, b_destination)
            {
                //Don't forget to set the EP square!
                return true;
            }
        }

        // Capture Left
        if self.move_id < 40 {
            if self.white_to_move() {
                b_destination = self.b_current_piece << 9;
                b_can_capture_left = B_NOT_UL_EDGE;
                b_opponent = self.b_black;
                b_not_last_rank = B_NOT_TOP_EDGE;
            } else {
                b_destination = self.b_current_piece >> 9;
                b_can_capture_left = B_NOT_DR_EDGE;
                b_opponent = self.b_white;
                b_not_last_rank = B_NOT_BOTTOM_EDGE;
            }
            if (self.b_current_piece & b_can_capture_left != 0) && 
                (b_destination & b_opponent != 0) && 
                self.add_move_if_valid(child, b_destination) 
            {
                if b_destination & b_not_last_rank != 0 {
                    self.move_id = 40;
                } else {
                    self.generate_promotion_moves(child, 30);
                }
                return true;
            }
        }

        // Capture Right
        if self.move_id < 50 {
            if self.white_to_move() {
                b_destination = self.b_current_piece << 7;
                b_can_capture_right = B_NOT_UR_EDGE;
                b_opponent = self.b_black;
                b_not_last_rank = B_NOT_TOP_EDGE;
            } else {
                b_destination = self.b_current_piece >> 7;
                b_can_capture_right = B_NOT_DL_EDGE;
                b_opponent = self.b_white;
                b_not_last_rank = B_NOT_BOTTOM_EDGE;
            }
            if (self.b_current_piece & b_can_capture_right != 0) && 
                (b_destination & b_opponent != 0) && 
                self.add_move_if_valid(child, b_destination) 
            {
                if b_destination & b_not_last_rank != 0 {
                    self.move_id = 50;
                } else {
                    self.generate_promotion_moves(child, 40);
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
    fn pawn_moves_black_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("h7".to_string());

        // h7 to h6
        let fen = String::from("rnbqkbnr/ppppppp1/7p/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,20);

        // h7 to h5
        let fen = String::from("rnbqkbnr/ppppppp1/8/7p/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g7".to_string()));
        assert_eq!(r.move_id,10);

        // g7 to g6
        let fen = String::from("rnbqkbnr/pppppp1p/6p1/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g7".to_string()));
        assert_eq!(r.move_id,20);

        // g7 to g5
        let fen = String::from("rnbqkbnr/pppppp1p/8/6p1/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f7".to_string()));
        assert_eq!(r.move_id,10);

        // f7 to f6
        let fen = String::from("rnbqkbnr/ppppp1pp/5p2/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f7".to_string()));
        assert_eq!(r.move_id,20);

        // f7 to f5
        let fen = String::from("rnbqkbnr/ppppp1pp/8/5p2/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,10);

        // e7 to e6
        let fen = String::from("rnbqkbnr/pppp1ppp/4p3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,20);

        // e7 to e5
        let fen = String::from("rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d7".to_string()));
        assert_eq!(r.move_id,10);

        // d7 to d6
        let fen = String::from("rnbqkbnr/ppp1pppp/3p4/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d7".to_string()));
        assert_eq!(r.move_id,20);

        // d7 to d5
        let fen = String::from("rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,10);

        // c7 to c6
        let fen = String::from("rnbqkbnr/pp1ppppp/2p5/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,20);

        // c7 to c5
        let fen = String::from("rnbqkbnr/pp1ppppp/8/2p5/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,10);

        // b7 to b6
        let fen = String::from("rnbqkbnr/p1pppppp/1p6/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,20);

        // b7 to b5
        let fen = String::from("rnbqkbnr/p1pppppp/8/1p6/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a7".to_string()));
        assert_eq!(r.move_id,10);

        // a7 to a6
        let fen = String::from("rnbqkbnr/1ppppppp/p7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a7".to_string()));
        assert_eq!(r.move_id,20);

        // a7 to a5
        let fen = String::from("rnbqkbnr/1ppppppp/8/p7/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a7".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h8".to_string()));
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
    fn pawn_moves_black_captures() {
        let mut r = prep_board("k7/5p1p/1p4p1/P1P4P/3pp3/2P2P2/8/K7 b - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("e4".to_string());

        // e4 to e3
        let fen = String::from("k7/5p1p/1p4p1/P1P4P/3p4/2P1pP2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e4".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e4 to f3
        let fen = String::from("k7/5p1p/1p4p1/P1P4P/3p4/2P2p2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e4".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d4".to_string()));
        assert_eq!(r.move_id,10);

        // d4 to d3
        let fen = String::from("k7/5p1p/1p4p1/P1P4P/4p3/2Pp1P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d4".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // d4 to c3
        let fen = String::from("k7/5p1p/1p4p1/P1P4P/4p3/2p2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d4".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g6".to_string()));
        assert_eq!(r.move_id,10);

        // g6 to g5
        let fen = String::from("k7/5p1p/1p6/P1P3pP/3pp3/2P2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g6".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // g6 to h5
        let fen = String::from("k7/5p1p/1p6/P1P4p/3pp3/2P2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g6".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,10);

        // b6 to b5
        let fen = String::from("k7/5p1p/6p1/PpP4P/3pp3/2P2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // b6 to c5
        let fen = String::from("k7/5p1p/6p1/P1p4P/3pp3/2P2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // b6 to a5
        let fen = String::from("k7/5p1p/6p1/p1P4P/3pp3/2P2P2/8/K7 w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
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
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // h7 to h8 Bishop
        let fen = String::from("k2r3B/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // h7 to h8 Rook
        let fen = String::from("k2r3R/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,2);
        assert_eq!(child.in_check,0);

        // h7 to h8 Queen
        let fen = String::from("k2r3Q/2P1P3/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,6);
        assert_eq!(child.in_check,0);

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
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // e7 to e8 Bishop
        let fen = String::from("k2rB3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // e7 to e8 Rook
        let fen = String::from("k2rR3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,2);
        assert_eq!(child.in_check,0);

        // e7 to e8 Queen
        let fen = String::from("k2rQ3/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,6);
        assert_eq!(child.in_check,0);

        // e7 to d8 Knight
        let fen = String::from("k2N4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,5);
        assert_eq!(child.in_check,0);

        // e7 to d8 Bishop
        let fen = String::from("k2B4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,5);
        assert_eq!(child.in_check,0);

        // e7 to d8 Rook
        let fen = String::from("k2R4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,33);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,7);
        assert_eq!(child.in_check,1);

        // e7 to d8 Queen
        let fen = String::from("k2Q4/2P4P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e7".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,11);
        assert_eq!(child.in_check,1);

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
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // c7 to c8 Bishop
        let fen = String::from("k1Br4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // c7 to c8 Rook
        let fen = String::from("k1Rr4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,2);
        assert_eq!(child.in_check,1);

        // c7 to c8 Queen
        let fen = String::from("k1Qr4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,6);
        assert_eq!(child.in_check,1);

        // c7 to d8 Knight
        let fen = String::from("k2N4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,5);
        assert_eq!(child.in_check,0);

        // c7 to d8 Bishop
        let fen = String::from("k2B4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,5);
        assert_eq!(child.in_check,0);

        // c7 to d8 Rook
        let fen = String::from("k2R4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,43);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,7);
        assert_eq!(child.in_check,1);

        // c7 to d8 Queen
        let fen = String::from("k2Q4/4P2P/8/8/8/8/8/K7 b - - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c7".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,11);
        assert_eq!(child.in_check,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(r.move_id,10);

    }

    #[test]
    fn pawn_moves_black_promotion() {
        let mut r = prep_board("7k/8/8/8/8/8/p2p1p2/4R2K b - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("f2".to_string());

        // f2 to f1 Knight
        let fen = String::from("7k/8/8/8/8/8/p2p4/4Rn1K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // f2 to f1 Bishop
        let fen = String::from("7k/8/8/8/8/8/p2p4/4Rb1K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // f2 to f1 Rook
        let fen = String::from("7k/8/8/8/8/8/p2p4/4Rr1K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-2);
        assert_eq!(child.in_check,1);

        // f2 to f1 Queen
        let fen = String::from("7k/8/8/8/8/8/p2p4/4Rq1K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-6);
        assert_eq!(child.in_check,1);

        // f2 to e1 Knight
        let fen = String::from("7k/8/8/8/8/8/p2p4/4n2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-5);
        assert_eq!(child.in_check,0);

        // f2 to e1 Bishop
        let fen = String::from("7k/8/8/8/8/8/p2p4/4b2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-5);
        assert_eq!(child.in_check,0);

        // f2 to e1 Rook
        let fen = String::from("7k/8/8/8/8/8/p2p4/4r2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,43);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-7);
        assert_eq!(child.in_check,1);

        // f2 to e1 Queen
        let fen = String::from("7k/8/8/8/8/8/p2p4/4q2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f2".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-11);
        assert_eq!(child.in_check,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,10);

        // d2 to d1 Knight
        let fen = String::from("7k/8/8/8/8/8/p4p2/3nR2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // d2 to d1 Bishop
        let fen = String::from("7k/8/8/8/8/8/p4p2/3bR2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // d2 to d1 Rook
        let fen = String::from("7k/8/8/8/8/8/p4p2/3rR2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-2);
        assert_eq!(child.in_check,0);

        // d2 to d1 Queen
        let fen = String::from("7k/8/8/8/8/8/p4p2/3qR2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-6);
        assert_eq!(child.in_check,0);

        // d2 to e1 Knight
        let fen = String::from("7k/8/8/8/8/8/p4p2/4n2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-5);
        assert_eq!(child.in_check,0);

        // d2 to e1 Bishop
        let fen = String::from("7k/8/8/8/8/8/p4p2/4b2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-5);
        assert_eq!(child.in_check,0);

        // d2 to e1 Rook
        let fen = String::from("7k/8/8/8/8/8/p4p2/4r2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,33);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-7);
        assert_eq!(child.in_check,1);

        // d2 to e1 Queen
        let fen = String::from("7k/8/8/8/8/8/p4p2/4q2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d2".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-11);
        assert_eq!(child.in_check,1);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,10);

        // a2 to a1 Knight
        let fen = String::from("7k/8/8/8/8/8/3p1p2/n3R2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // a2 to a1 Bishop
        let fen = String::from("7k/8/8/8/8/8/3p1p2/b3R2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,0);
        assert_eq!(child.in_check,0);

        // a2 to a1 Rook
        let fen = String::from("7k/8/8/8/8/8/3p1p2/r3R2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-2);
        assert_eq!(child.in_check,0);

        // a2 to a1 Queen
        let fen = String::from("7k/8/8/8/8/8/3p1p2/q3R2K w - - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);
        assert_eq!(child.material,-6);
        assert_eq!(child.in_check,0);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h8".to_string()));
        assert_eq!(r.move_id,10);

    }
}
