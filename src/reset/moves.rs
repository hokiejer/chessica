use crate::reset::Reset;
use crate::bitops;
use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_LOWER_RIGHT_CORNER;
use crate::reset::r#const::B_LOWER_LEFT_CORNER;
use crate::reset::r#const::B_UPPER_RIGHT_CORNER;

impl Reset {

    /// Prepare a Reset to generate moves
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn initialize_move_generation(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::lowest_bit(self.b_white);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black);
        }
        self.move_id = 10;	//Prime the first move
    }

    /// Consider the next moveable piece
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn consider_next_moveable_piece(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::next_lowest_bit(self.b_white, self.b_current_piece);
        } else {
            self.b_current_piece = bitops::next_lowest_bit(self.b_black, self.b_current_piece);
        }
        self.move_id = 10;
    }


    /// Generate the next move for a Reset
    ///
    /// Returns Boolean indicating `true` if move options have not been exhausted
    /// and `false` if they have.  Expects the child reset to already be initialized 
    /// from the parent.
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let mut child = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// r.generate_next_move(&mut child);
    /// ```
    pub fn generate_next_move(&mut self, child: &mut Reset) -> bool {
        while self.b_current_piece != 0 {
            if self.b_current_piece & self.b_pawns != 0 { // Pawn
                if self.generate_next_pawn_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_knights != 0 { // Knight
                if self.generate_next_knight_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_bishops != 0 { // Bishop
                if self.generate_next_bishop_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_rooks != 0 { // Rook
                if self.generate_next_rook_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_queens != 0 { // Queen
                if self.generate_next_queen_move(child) {
                    break;
                }
            } else { // King
                if self.generate_next_king_move(child) {
                    break;
                }
            }

            // do stuff
            self.consider_next_moveable_piece();
        }
        self.b_current_piece > 0
    }


    /// Adds a move to the specified child reset if valid
    ///
    pub fn add_move_if_valid(&mut self, child: &mut Reset, b_destination: u64) -> bool {

        use crate::utils;
        println!("In add_move_if_valid ({} to {})", utils::convert_bitstring_to_square(self.b_current_piece),utils::convert_bitstring_to_square(b_destination));
        self.init_child(child);
        child.b_from = self.b_current_piece;
        child.b_to = b_destination;

        if child.b_to & child.b_all != 0 { // Capture
            println!("Capture Processing!!");
            child.capture_processing();
        }
        child.b_all &= !child.b_from;
        child.b_all |= child.b_to;
        if self.white_to_move() {
            child.b_white &= !child.b_from;
            child.b_white |= child.b_to;
        } else {
            child.b_black &= !child.b_from;
            child.b_black |= child.b_to;
        }
        child.b_white &= !child.b_from;
        child.b_black &= !child.b_from;
        if child.b_from & child.b_pawns != 0 {
            println!("Pawn move");
            child.b_pawns &= !child.b_from;
            child.b_pawns |= child.b_to;
            child.halfmove_clock = 0; // Resets on pawn move
        } else if child.b_from & child.b_knights != 0 {
            println!("Knight move");
            child.b_knights &= !child.b_from;
            child.b_knights |= child.b_to;
        } else if child.b_from & child.b_bishops != 0 {
            println!("Bishop move");
            child.b_bishops &= !child.b_from;
            child.b_bishops |= child.b_to;
        } else if child.b_from & child.b_rooks != 0 {
            println!("Rook move");
            child.b_rooks &= !child.b_from;
            child.b_rooks |= child.b_to;
            if child.b_from & B_FOUR_CORNERS != 0 {
                if child.b_to & B_LOWER_RIGHT_CORNER != 0 {
                    child.white_castle_k = 0;
                } else if child.b_to & B_LOWER_LEFT_CORNER != 0 {
                    child.white_castle_q = 0;
                } else if child.b_to & B_UPPER_RIGHT_CORNER != 0 {
                    child.black_castle_k = 0;
                } else { // B_UPPER_RIGHT_CORNER
                    child.black_castle_q = 0;
                }
            }
        } else if child.b_from & child.b_queens != 0 {
            println!("Queen move");
            child.b_queens &= !child.b_from;
            child.b_queens |= child.b_to;
        } else {
            println!("King move");
            child.b_kings &= !child.b_from;
            child.b_kings |= child.b_to;
            child.white_castle_k = 0;
            child.white_castle_q = 0;
            child.black_castle_k = 0;
            child.black_castle_q = 0;
        }
        // Move is invalid if I'm moving into check
        if self.white_to_move() {
            println!("White move - safety check");
            if !child.white_is_safe(child.b_kings & child.b_white) {
                println!("White is not safe!");
                return false;
            }
            println!("White is safe!");
            if !child.black_is_safe(child.b_kings & child.b_black) {
                child.in_check = 1;
            }
        } else {
            println!("Black move - safety check");
            if !child.black_is_safe(child.b_kings & child.b_black) {
                return false;
            }
            println!("Black is safe!");
            if !child.white_is_safe(child.b_kings & child.b_white) {
                child.in_check = 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::utils;

    #[test]
    fn move_init_move_generation() {
        let mut r = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000001,"b_current_piece");
        assert_eq!(r.move_id,10,"move_id");

        let mut r = reset::new();
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        r.init_from_fen(fen2.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0001000000000000,"b_current_piece");
        assert_eq!(r.move_id,10,"move_id");
    }

    #[test]
    fn move_consider_next_moveable_piece() {
        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 w Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000008);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000080);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);

        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 b Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0100000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0800000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);
    }

    #[test]
    fn white_cannot_move_into_check() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "k7/3r4/8/8/7P/3N4/8/3K4 w - - 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d3".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("b2".to_string()));
        assert!(!result);
    }

    #[test]
    fn black_cannot_move_into_check() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "rnbqk1nr/ppppbppp/8/8/3P4/8/PPP1QPPP/RNB1KBNR b KQkq d3 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("e7".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("b4".to_string()));
        assert!(!result);
    }

    #[test]
    fn white_check_detected() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "r1bqkbnr/ppp2ppp/2np4/4pN2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("f5".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("g7".to_string()));
        assert!(result);
        assert_eq!(child.in_check,1);
    }

    #[test]
    fn black_check_detected() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "rnbqk1nr/ppppbppp/8/8/3P4/8/PPP1BPPP/RNBQK1NR b KQkq - 1 2";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("e7".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("b4".to_string()));
        child.print();
        assert!(result);
        assert_eq!(child.in_check,1);
    }

    #[test]
    fn white_check_midgame() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "rnb1kb1r/p2p1ppp/5n2/1p3NqP/4PpP1/3P4/PPP5/RNBQ1KR1 w kq - 1 14";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("f5".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("g7".to_string()));
        child.print();
        assert!(result);
        assert_eq!(child.in_check,1);
    }

    #[test]
    fn black_check_midgame() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "rnb1kb1r/p2p1ppp/5n2/1p3NqP/4P1P1/3P4/PPP5/RNBQ1KR1 b kq - 1 14";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("g5".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()));
        child.print();
        assert!(result);
        assert_eq!(child.in_check,1);
    }

}

