use crate::reset::Reset;
use crate::reset::pinned::PIN_MATCH_NS;
use crate::reset::pinned::PIN_MATCH_NESW;
use crate::reset::pinned::PIN_MATCH_SENW;
use crate::reset::r#const::B_NOT_N_EDGE;
use crate::reset::r#const::B_NOT_S_EDGE;
use crate::reset::r#const::B_NOT_NW_EDGE;
use crate::reset::r#const::B_NOT_NE_EDGE;
use crate::reset::r#const::B_NOT_SW_EDGE;
use crate::reset::r#const::B_NOT_SE_EDGE;
use crate::reset::r#const::B_RANK_2;
use crate::reset::r#const::B_RANK_7;

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
                child.material += 8 * multiplier;
                self.move_id = move_base + 10;
            },
            _ => panic!("Shouldn't get here!"),
        }
        if self.white_to_move() {
            if !child.black_is_safe(child.b_kings & child.b_black()) {
                child.in_check = 1;
            }
        } else if !child.white_is_safe(child.b_kings & child.b_white) {
            child.in_check = 1;
        }
    }

    pub fn white_en_passant_cleanup(&mut self, child: &mut Reset) -> bool {
        let b_pawn_to_remove = self.b_en_passant >> 8;
        child.b_all &= !b_pawn_to_remove;
        child.b_pawns &= !b_pawn_to_remove;
        child.material += 1;
        child.capture = 1;
        if !child.white_is_safe(child.b_kings & child.b_white) {
            return false;
        }
        if !child.black_is_safe(child.b_kings & child.b_black()) {
            child.in_check = 1;
        }
        self.valid_child_post_processing(child);
        return true;
    }

    pub fn generate_next_white_pawn_move(&mut self, child: &mut Reset) -> bool {
        let mut b_destination: u64;

        // Forward one (North)
        if self.move_id < 20 {
            b_destination = self.b_current_piece << 8;
            if self.b_current_piece & B_NOT_N_EDGE != 0 &&
                (b_destination & self.b_all == 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_NS) 
            {
                if b_destination & B_NOT_N_EDGE != 0 {
                    self.move_id = 20;
                } else {
                    self.generate_promotion_moves(child, 10);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }
                
        // Forward two (North)
        if self.move_id < 30 {
            self.move_id = 30;
            let b_forward_one: u64 = self.b_current_piece << 8;
            b_destination = self.b_current_piece << 16;

            if (self.b_current_piece & B_RANK_2 != 0) &&
                ((b_forward_one & self.b_all) == 0) &&
                ((b_destination & self.b_all) == 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_NS)
            {
                child.b_en_passant = b_forward_one;
                self.valid_child_post_processing(child);
                return true;
            }
        } 

        // Capture Left (Northwest)
        if self.move_id < 40 {
            b_destination = self.b_current_piece << 9;
            if (self.b_current_piece & B_NOT_NW_EDGE != 0) && 
                (b_destination & self.b_black() != 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_SENW) 
            {
                if b_destination & B_NOT_N_EDGE != 0 {
                    self.move_id = 40;
                } else {
                    self.generate_promotion_moves(child, 30);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Capture Right (Northeast)
        if self.move_id < 50 {
            b_destination = self.b_current_piece << 7;
            if (self.b_current_piece & B_NOT_NE_EDGE != 0) && 
                (b_destination & self.b_black() != 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_NESW) 
            {
                if b_destination & B_NOT_N_EDGE != 0 {
                    self.move_id = 50;
                } else {
                    self.generate_promotion_moves(child, 40);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }

        if self.b_en_passant != 0 {
            // Capture Left (Northwest) En Passant
            b_destination = self.b_current_piece << 9;
            if b_destination == self.b_en_passant &&
                (self.b_current_piece & B_NOT_NW_EDGE != 0)
            {
                self.add_move_unconditional(child, b_destination);
                self.consider_next_moveable_piece();
                return self.white_en_passant_cleanup(child);
            }

            // Capture Right (Northeast) En Passant
            b_destination = self.b_current_piece << 7;
            if b_destination == self.b_en_passant &&
                (self.b_current_piece & B_NOT_NE_EDGE != 0)
            {
                self.add_move_unconditional(child, b_destination);
                self.consider_next_moveable_piece();
                return self.white_en_passant_cleanup(child);
            }
        }

        self.consider_next_moveable_piece();
        false
    }

    pub fn black_en_passant_cleanup(&mut self, child: &mut Reset) -> bool {
        let b_pawn_to_remove = self.b_en_passant << 8;
        child.b_all &= !b_pawn_to_remove;
        child.b_white &= !b_pawn_to_remove;
        child.b_pawns &= !b_pawn_to_remove;
        child.material -= 1;
        child.capture = 1;
        if !child.black_is_safe(child.b_kings & child.b_black()) {
            return false;
        }
        if !child.white_is_safe(child.b_kings & child.b_white) {
            child.in_check = 1;
        }
        self.valid_child_post_processing(child);
        true
    }

    pub fn generate_next_black_pawn_move(&mut self, child: &mut Reset) -> bool {
        let mut b_destination: u64;

        // Forward one (South)
        if self.move_id < 20 {
            b_destination = self.b_current_piece >> 8;
            if self.b_current_piece & B_NOT_S_EDGE != 0 &&
                (b_destination & self.b_all == 0) &&
                self.add_move_if_valid(child, b_destination,PIN_MATCH_NS) 
            {
                if b_destination & B_NOT_S_EDGE != 0 {
                    self.move_id = 20;
                } else {
                    self.generate_promotion_moves(child, 10);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }
                
        // Forward two (South)
        if self.move_id < 30 {
            self.move_id = 30;
            let b_forward_one: u64 = self.b_current_piece >> 8;
            b_destination = self.b_current_piece >> 16;

            if (self.b_current_piece & B_RANK_7 != 0) &&
                ((b_forward_one & self.b_all) == 0) &&
                ((b_destination & self.b_all) == 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_NS)
            {
                child.b_en_passant = b_forward_one;
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Capture Left (Southeast)
        if self.move_id < 40 {
            b_destination = self.b_current_piece >> 9;
            if (self.b_current_piece & B_NOT_SE_EDGE != 0) && 
                (b_destination & self.b_white != 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_SENW) 
            {
                if b_destination & B_NOT_S_EDGE != 0 {
                    self.move_id = 40;
                } else {
                    self.generate_promotion_moves(child, 30);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }

        // Capture Right (Southwest)
        if self.move_id < 50 {
            b_destination = self.b_current_piece >> 7;
            if (self.b_current_piece & B_NOT_SW_EDGE != 0) && 
                (b_destination & self.b_white != 0) &&
                self.add_move_if_valid(child, b_destination, PIN_MATCH_NESW) 
            {
                if b_destination & B_NOT_S_EDGE != 0 {
                    self.move_id = 50;
                } else {
                    self.generate_promotion_moves(child, 40);
                }
                self.valid_child_post_processing(child);
                return true;
            }
        }

        if self.b_en_passant != 0 {
            // Capture Left (Southeast) En Passant
            b_destination = self.b_current_piece >> 9;
            if b_destination == self.b_en_passant &&
                (self.b_current_piece & B_NOT_SE_EDGE != 0)
            {
                self.add_move_unconditional(child, b_destination);
                self.consider_next_moveable_piece();
                return self.black_en_passant_cleanup(child);
            }

            // Capture Right (Southwest) En Passant
            b_destination = self.b_current_piece >> 7;
            if b_destination == self.b_en_passant &&
                (self.b_current_piece & B_NOT_SW_EDGE != 0)
            {
                self.add_move_unconditional(child, b_destination);
                self.consider_next_moveable_piece();
                return self.black_en_passant_cleanup(child);
            }
        }

        self.consider_next_moveable_piece();
        false
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
        r
    }

    #[test]
    fn pawn_moves_white_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("h2");

        // h2 to h3
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h2".to_string()));
        assert_eq!(r.move_id,20);

        // h2 to h4
        let fen = String::from("rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1");
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
        let fen = String::from("rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a2".to_string()));
        assert_eq!(r.move_id,30);

        // No more moves for this pawn
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);

    }

    #[test]
    fn pawn_moves_black_first_starting_position() {
        let mut r = prep_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        let mut child = reset::new();
        r.current_piece_init("h7");

        // h7 to h6
        let fen = String::from("rnbqkbnr/ppppppp1/7p/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h7".to_string()));
        assert_eq!(r.move_id,20);

        // h7 to h5
        let fen = String::from("rnbqkbnr/ppppppp1/8/7p/8/8/PPPPPPPP/RNBQKBNR w KQkq h6 0 2");
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
        let fen = String::from("rnbqkbnr/pppppp1p/8/6p1/8/8/PPPPPPPP/RNBQKBNR w KQkq g6 0 2");
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
        let fen = String::from("rnbqkbnr/ppppp1pp/8/5p2/8/8/PPPPPPPP/RNBQKBNR w KQkq f6 0 2");
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
        let fen = String::from("rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq e6 0 2");
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
        let fen = String::from("rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq d6 0 2");
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
        let fen = String::from("rnbqkbnr/pp1ppppp/8/2p5/8/8/PPPPPPPP/RNBQKBNR w KQkq c6 0 2");
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
        let fen = String::from("rnbqkbnr/p1pppppp/8/1p6/8/8/PPPPPPPP/RNBQKBNR w KQkq b6 0 2");
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
        let fen = String::from("rnbqkbnr/1ppppppp/8/p7/8/8/PPPPPPPP/RNBQKBNR w KQkq a6 0 2");
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
        r.current_piece_init("g3");

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
        r.current_piece_init("e4");

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
        r.current_piece_init("h7");

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

    }

    #[test]
    fn pawn_moves_black_promotion() {
        let mut r = prep_board("7k/8/8/8/8/8/p2p1p2/4R2K b - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("f2");

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

    #[test]
    fn pawn_moves_white_ep_capture_right() {
        let mut r = prep_board("rnbqkbnr/ppppp1p1/8/5pPp/8/8/PPPPPP1P/RNBQKBNR w KQkq f6 0 1");
        let mut child = reset::new();
        r.current_piece_init("g5");

        // g5 to g6
        let fen = String::from("rnbqkbnr/ppppp1p1/6P1/5p1p/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g5".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // g5 to f6 (EP)
        let fen = String::from("rnbqkbnr/ppppp1p1/5P2/7p/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,1);
    }

    #[test]
    fn pawn_moves_white_ep_capture_left() {
        let mut r = prep_board("rnbqkbnr/ppppp1p1/8/5pPp/8/8/PPPPPP1P/RNBQKBNR w KQkq h6 0 1");
        let mut child = reset::new();
        r.current_piece_init("g5");

        // g5 to g6
        let fen = String::from("rnbqkbnr/ppppp1p1/6P1/5p1p/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("g5".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // g5 to h6 (EP)
        let fen = String::from("rnbqkbnr/ppppp1p1/7P/5p2/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,1);
    }

    #[test]
    fn pawn_moves_white_ep_capture_to_escape_check() {
        let mut r = prep_board("8/8/8/pP6/1K1k2p1/6P1/7P/8 w - a6 0 3");
        let mut child = reset::new();
        r.current_piece_init("b5");

        // g5 to g6
        let fen = String::from("8/8/P7/8/1K1k2p1/6P1/7P/8 b - - 0 3");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(child.capture,1);
    }

    #[test]
    fn pawn_moves_black_ep_capture_right() {
        let mut r = prep_board("rnbqkb1r/pppp1ppp/7n/8/3PpP2/8/PPP1P1PP/RNBQKBNR b KQkq d3 0 1");
        let mut child = reset::new();
        r.current_piece_init("e4");

        // e4 to e3
        let fen = String::from("rnbqkb1r/pppp1ppp/7n/8/3P1P2/4p3/PPP1P1PP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e4".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e4 to f3 (EP)
        let fen = String::from("rnbqkb1r/pppp1ppp/7n/8/5P2/3p4/PPP1P1PP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h6".to_string()));
        assert_eq!(r.move_id,10);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-1);
    }

    #[test]
    fn pawn_moves_black_ep_capture_left() {
        let mut r = prep_board("rnbqkb1r/pppp1ppp/7n/8/3PpP2/8/PPP1P1PP/RNBQKBNR b KQkq f3 0 1");
        let mut child = reset::new();
        r.current_piece_init("e4");

        // e4 to e3
        let fen = String::from("rnbqkb1r/pppp1ppp/7n/8/3P1P2/4p3/PPP1P1PP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e4".to_string()));
        assert_eq!(r.move_id,20);
        assert_eq!(child.capture,0);

        // e4 to d3
        let fen = String::from("rnbqkb1r/pppp1ppp/7n/8/3P4/5p2/PPP1P1PP/RNBQKBNR w KQkq - 0 2");
        let retval = r.generate_next_pawn_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h6".to_string()));
        assert_eq!(r.move_id,10);
        assert_eq!(child.capture,1);
        assert_eq!(child.material,-1);
    }

}
