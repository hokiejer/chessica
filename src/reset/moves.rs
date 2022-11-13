use std::process;
use crate::reset::Reset;
use crate::bitops;
use crate::reset::pinned::PIN_DIMENSION_UNSET;

use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_SE_CORNER;
use crate::reset::r#const::B_SW_CORNER;
use crate::reset::r#const::B_NE_CORNER;

use crate::reset::r#const::BLACK;
use crate::reset::r#const::WHITE;

use crate::bitops::r#const::U8_NOT_BIT1;
use crate::bitops::r#const::U8_NOT_BIT2;
use crate::bitops::r#const::U8_NOT_BIT3;
use crate::bitops::r#const::U8_NOT_BIT4;
use crate::bitops::r#const::U8_NOT_BIT1_OR_BIT2;
use crate::bitops::r#const::U8_NOT_BIT3_OR_BIT4;

impl Reset {

    /// Prepare a Reset to generate moves.  This is called from both `init_from_fen` and after a
    /// child is created by `generate_next_move`.  That way any new Reset is ready to generate
    /// moves.
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
            self.bi_current_piece = bitops::get_bit_number(self.b_current_piece);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black());
            self.bi_current_piece = bitops::get_bit_number(self.b_current_piece);
        }
        self.pin_dimension = PIN_DIMENSION_UNSET;
        self.move_id = 10;	//Prime the first move
    }

    /// Consider the next moveable piece
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn consider_next_moveable_piece(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::next_lowest_bit(self.b_white, self.b_current_piece);
            self.bi_current_piece = bitops::get_bit_number(self.b_current_piece);
        } else {
            self.b_current_piece = bitops::next_lowest_bit(self.b_black(), self.b_current_piece);
            self.bi_current_piece = bitops::get_bit_number(self.b_current_piece);
        }
        self.pin_dimension = PIN_DIMENSION_UNSET;
        self.move_id = 10;
    }

    /// Generate the next move for a Reset
    ///
    /// Returns Boolean indicating `true` if a move was successfully returned
    /// and `false` if no moves remain.  Expects the child reset to already be 
    /// initialized from the parent.
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let mut child = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.generate_next_move(&mut child);
    /// ```
    pub fn generate_next_move(&mut self, child: &mut Reset) -> bool {
        let mut found_move: bool = false;
        while self.b_current_piece != 0 {
            if self.b_current_piece & self.b_pawns != 0 { // Pawn
                if self.generate_next_pawn_move(child) {
                    found_move = true;
                    break;
                }
            } else if self.b_current_piece & self.b_knights != 0 { // Knight
                if self.generate_next_knight_move(child) {
                    found_move = true;
                    break;
                }
            } else if self.b_current_piece & self.b_bishops != 0 { // Bishop
                if self.generate_next_bishop_move(child) {
                    found_move = true;
                    break;
                }
            } else if self.b_current_piece & self.b_rooks != 0 { // Rook
                if self.generate_next_rook_move(child) {
                    found_move = true;
                    break;
                }
            } else if self.b_current_piece & self.b_kings != 0 { // King
                if self.generate_next_king_move(child) {
                    found_move = true;
                    break;
                }
            } else { // Queen
                if self.generate_next_queen_move(child) {
                    found_move = true;
                    break;
                }
            }
        }
        if found_move {
            true
        } else {
            false
        }
    }

    /// Adds a move to the specified child - MAY BE INVALID
    /// Does not set `in_check` - expects the caller to look for safety later
    ///
    pub fn add_move_unconditional(&mut self, child: &mut Reset, b_destination: u64) {

        self.init_child(child);
        child.b_from = self.b_current_piece;
        child.b_to = b_destination;
        child.bi_from = bitops::get_bit_number(child.b_from);
        child.bi_to = bitops::get_bit_number(child.b_to);

        if child.b_to & child.b_all != 0 { // Capture
            #[cfg(debug_assertions)]
            if child.b_to & self.b_kings != 0 {
                println!("King was captured!?!?!");
                println!("Self:");
                self.print();
                println!("Child:");
                child.print();
                process::abort();
            }
            self.capture_processing(child);
        }
        child.b_all &= !child.b_from;
        child.b_all |= child.b_to;
        if self.white_to_move() {
            child.b_white &= !child.b_from;
            child.b_white |= child.b_to;
        }

        if child.b_from & child.b_pawns != 0 {
            child.b_pawns &= !child.b_from;
            child.b_pawns |= child.b_to;
            child.halfmove_clock = 0; // Resets on pawn move
        } else if child.b_from & child.b_knights != 0 {
            child.b_knights &= !child.b_from;
            child.b_knights |= child.b_to;
        } else if child.b_from & child.b_bishops != 0 {
            child.b_bishops &= !child.b_from;
            child.b_bishops |= child.b_to;
        } else if child.b_from & child.b_rooks != 0 {
            child.b_rooks &= !child.b_from;
            child.b_rooks |= child.b_to;
            if child.b_from & B_FOUR_CORNERS != 0 {
                if child.b_from & B_SE_CORNER != 0 {
                    //white_castle_k = 0;
                    child.castle_bits &= U8_NOT_BIT1;
                } else if child.b_from & B_SW_CORNER != 0 {
                    //white_castle_q = 0;
                    child.castle_bits &= U8_NOT_BIT2;
                } else if child.b_from & B_NE_CORNER != 0 {
                    //black_castle_k = 0;
                    child.castle_bits &= U8_NOT_BIT3;
                } else { // B_NW_CORNER
                    //black_castle_q = 0;
                    child.castle_bits &= U8_NOT_BIT4;
                }
            }
        } else if child.b_from & child.b_kings != 0 {
            child.b_kings &= !child.b_from;
            child.b_kings |= child.b_to;
            if self.white_to_move() {
                child.white_king_square = bitops::get_bit_number(child.b_to);
                child.castle_bits &= U8_NOT_BIT1_OR_BIT2;
            } else {
                child.black_king_square = bitops::get_bit_number(child.b_to);
                child.castle_bits &= U8_NOT_BIT3_OR_BIT4;
            }
        } else {
            // Queen moved
        }
    }

    /// Adds a move to the specified child reset if valid
    /// Uses a minimal safety check (if possible)
    ///
    pub fn add_move_if_valid(&mut self, child: &mut Reset, b_destination: u64, pin_match: u8) -> bool {

        if self.pin_dimension == PIN_DIMENSION_UNSET {
            self.set_current_piece_pin_dimension();
        } 
        if self.pin_dimension & pin_match != 0 {
            return false;
        }
        self.add_move_unconditional(child, b_destination);

        // Move is invalid if I'm moving into check
        if self.white_to_move() {
            if self.in_check != 0 {
                if !child.white_is_safe(child.b_kings & child.b_white) {
                    return false;
                }
            }
            if !child.is_safe_from_revealed_check(child.black_king_square,child.bi_from,BLACK) ||
                !child.is_safe_from_direct_check(child.black_king_square,child.bi_to,BLACK) 
            {
                child.in_check = 1;
            }
        } else {
            if self.in_check != 0 {
                if !child.black_is_safe(child.b_kings & child.b_black()) {
                    return false;
                }
            }
            if !child.is_safe_from_revealed_check(child.white_king_square,child.bi_from,WHITE) ||
                !child.is_safe_from_direct_check(child.white_king_square,child.bi_to,WHITE) 
            {
                child.in_check = 1;
            }
        }
        true
    }

    /// Adds a move to the specified child reset if valid
    /// Forces a full king safety check
    ///
    pub fn add_move_full_safety_check(&mut self, child: &mut Reset, b_destination: u64) -> bool {

        self.add_move_unconditional(child, b_destination);

        // Move is invalid if I'm moving into check
        if self.white_to_move() {
            if !child.white_is_safe(child.b_kings & child.b_white) {
                return false;
            }
            if !child.is_safe_from_revealed_check(child.black_king_square,child.bi_from,BLACK) ||
                !child.is_safe_from_direct_check(child.black_king_square,child.bi_to,BLACK) 
            {
                child.in_check = 1;
            }
        } else {
            if !child.black_is_safe(child.b_kings & child.b_black()) {
                return false;
            }
            if !child.is_safe_from_revealed_check(child.white_king_square,child.bi_from,WHITE) ||
                !child.is_safe_from_direct_check(child.white_king_square,child.bi_to,WHITE) 
            {
                child.in_check = 1;
            }
        }
        true
    }

    /// Processing to be done on a valid child before returning the move
    pub fn valid_child_post_processing(&mut self, child: &mut Reset) {
        if child.capture != 0 || child.b_to & child.b_pawns != 0 || child.promotion != 0 {
            child.halfmove_clock = 0;
        } else {
            child.halfmove_clock += 1;
        }
        if self.white_to_move() {
            child.to_move = 1;
        } else {
            //child.to_move = 0; This was already initialized to zero
            child.fullmove_number += 1;
        }
        child.initialize_move_generation();
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::utils;
    use crate::reset::Reset;
    use crate::reset::pinned::PIN_MATCH_NONE;
    use crate::reset::pinned::PIN_MATCH_NS;
    use crate::reset::pinned::PIN_MATCH_EW;
    use crate::reset::pinned::PIN_MATCH_NESW;
    use crate::reset::pinned::PIN_MATCH_SENW;
    use crate::reset::pinned::PIN_DIMENSION_NONE;
    use crate::reset::pinned::PIN_DIMENSION_NS;
    use crate::reset::pinned::PIN_DIMENSION_EW;
    use crate::reset::pinned::PIN_DIMENSION_NESW;
    use crate::reset::pinned::PIN_DIMENSION_SENW;

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
        assert_eq!(r.b_current_piece,0x0000000000000008);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000080);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);

        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 b Qk - 0 1";
        r.init_from_fen(fen.to_string());
        assert_eq!(r.b_current_piece,0x0100000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0800000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);
    }

    #[test]
    fn white_check_detected() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "r1bqkbnr/ppp2ppp/2np4/4pN2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("f5".to_string());
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("g7".to_string()),PIN_MATCH_NONE);
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
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("b4".to_string()),PIN_MATCH_NONE);
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
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("g7".to_string()),PIN_MATCH_NONE);
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
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NONE);
        child.print();
        assert!(result);
        assert_eq!(child.in_check,1);
    }

    #[test]
    fn pin_matching_ns() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "k1r1r3/8/8/8/8/8/3B4/3K4 w - - 0 1";

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NS;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NS);
        assert!(result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_EW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NS);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NESW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NS);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_SENW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NS);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NONE;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NS);
        assert!(result);
    }

    #[test]
    fn pin_matching_ew() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "k1r1r3/8/8/8/8/8/3B4/3K4 w - - 0 1";

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NS;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_EW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_EW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_EW);
        assert!(result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NESW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_EW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_SENW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_EW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NONE;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_EW);
        assert!(result);
    }

    #[test]
    fn pin_matching_nesw() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "k1r1r3/8/8/8/8/8/3B4/3K4 w - - 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NS;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NESW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_EW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NESW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NESW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NESW);
        assert!(result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_SENW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NESW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NONE;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_NESW);
        assert!(result);
    }

    #[test]
    fn pin_matching_senw() {
        let mut r = reset::new();
        let mut child = reset::new();
        let fen = "k1r1r3/8/8/8/8/8/3B4/3K4 w - - 0 1";
        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NS;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_SENW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_EW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_SENW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NESW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_SENW);
        assert!(!result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_SENW;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_SENW);
        assert!(result);

        r.init_from_fen(fen.to_string());
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.pin_dimension = PIN_DIMENSION_NONE;
        let result = r.add_move_if_valid(&mut child, utils::convert_square_to_bitstring("f4".to_string()),PIN_MATCH_SENW);
        assert!(result);
    }

    #[test]
    fn generate_next_move_board_1() {
        let mut r = reset::new();
        let mut child: Reset = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());

        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1");
        let result = r.generate_next_move(&mut child);
        assert!(!result);
    }

    #[test]
    fn generate_next_move_board_2() {
        let mut r = reset::new();
        let mut child: Reset = reset::new();
        let fen = "1rb2rqk/p3R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 b - - 0 1";
        //assert_eq!(child.to_fen(),"1rb2rqk/p3R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        //let result = r.generate_next_move(&mut child);
        r.init_from_fen(fen.to_string());

        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/p3R1pp/8/1p3BP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/p3R1p1/1p5p/5BP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/p3R1p1/1p6/5BPp/5P1Q/8/P4N1P/R5K1 w - h6 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/p3R2p/1p4p1/5BP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/4R1pp/pp6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2rqk/4R1pp/1p6/p4BP1/5P1Q/8/P4N1P/R5K1 w - a6 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3Rqpp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3R1pp/1p2q3/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3R1pp/1p6/3q1BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3R1pp/1p6/5BP1/2q2P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3R1pp/1p6/5BP1/5P1Q/1q6/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb2r1k/p3R1pp/1p6/5BP1/5P1Q/8/q4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb3qk/p3Rrpp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb3qk/p3R1pp/1p3r2/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb3qk/p3R1pp/1p6/5rP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rb1r1qk/p3R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1rbr2qk/p3R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1r3rqk/p2bR1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1r3rqk/p3R1pp/1p2b3/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1r3rqk/p3R1pp/1p6/5bP1/5P1Q/8/P4N1P/R5K1 w - - 0 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1r3rqk/pb2R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"1r3rqk/p3R1pp/bp6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"2b2rqk/pr2R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let _result = r.generate_next_move(&mut child);
        assert_eq!(child.to_fen(),"r1b2rqk/p3R1pp/1p6/5BP1/5P1Q/8/P4N1P/R5K1 w - - 1 2");
        let result = r.generate_next_move(&mut child);
        assert!(!result);
    }

}

