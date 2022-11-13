use crate::reset::Reset;
use crate::reset::pinned::PIN_MATCH_NS;
use crate::reset::pinned::PIN_MATCH_EW;
use crate::reset::pinned::PIN_MATCH_NESW;
use crate::reset::pinned::PIN_MATCH_SENW;
use crate::reset::r#const::WHITE;
use crate::reset::r#const::BLACK;

impl Reset {

    /// Generate the next possible rook move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_rook_move(&mut self, child: &mut Reset) -> bool {
        use crate::reset::r#const::B_NOT_N_EDGE;
        use crate::reset::r#const::B_NOT_E_EDGE;
        use crate::reset::r#const::B_NOT_W_EDGE;
        use crate::reset::r#const::B_NOT_S_EDGE;

        let b_available_moves: u64 = if self.white_to_move() {
            !self.b_white
        } else {
            self.b_white | !self.b_all
        };

        // North
        let next_line = 20;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << ((self.move_id % 10) * 8);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_N_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target <<= 8;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_NS) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    self.valid_child_post_processing(child);
                    return true;
                } else {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                        break;
                    }
                }
            }
        }

        // South
        let next_line = 30;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id % 10) * 8);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_S_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target >>= 8;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_NS) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    self.valid_child_post_processing(child);
                    return true;
                } else {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                        break;
                    }
                }
            }
        }

        // West
        let next_line = 40;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << (self.move_id % 10);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_W_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target <<= 1;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_EW) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    self.valid_child_post_processing(child);
                    return true;
                } else {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                        break;
                    }
                }
            }
        }

        // East
        let mut b_target = self.b_current_piece >> (self.move_id % 10);
        loop {
            // If we can't move any farther, give up on this line
            if b_target & B_NOT_E_EDGE == 0 {
                break;
            }
            b_target >>= 1;
            self.move_id += 1;
            // If my color is on the target, give up on this line
            if b_available_moves & b_target == 0 {
                break;
            }
            if self.add_move_if_valid(child, b_target, PIN_MATCH_EW) {
                // If this is a capture, we're done with this line
                if b_target & self.b_all != 0 {
                    self.consider_next_moveable_piece();
                }
                self.valid_child_post_processing(child);
                return true;
            } else {
                // If this is a capture, we're done with this line
                if b_target & self.b_all != 0 {
                    break;
                }
            }
        }

        self.consider_next_moveable_piece();
        false
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
    fn white_rook_moves_no_kings() {
        let mut r = prep_board("8/8/1R2r3/8/8/1r2R3/8/8 w - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("b6".to_string());

        // Up 1
        let fen = String::from("8/1R6/4r3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // Up 2
        let fen = String::from("1R6/8/4r3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // Down 1
        let fen = String::from("8/8/4r3/1R6/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);

        // Down 2
        let fen = String::from("8/8/4r3/8/1R6/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,22);
        assert_eq!(child.capture,0);

        // Down 3
        let fen = String::from("8/8/4r3/8/8/1R2R3/8/8 b - - 0 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,1);

        // Left 1
        let fen = String::from("8/8/R3r3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);

        // Right 1
        let fen = String::from("8/8/2R1r3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,0);

        // Right 2
        let fen = String::from("8/8/3Rr3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,0);

        // Right 3
        let fen = String::from("8/8/4R3/8/8/1r2R3/8/8 b - - 0 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000000000000000);
        assert_eq!(r.move_id,10);
        assert_eq!(child.capture,1);

    }

    #[test]
    fn black_rook_moves_no_kings() {
        let mut r = prep_board("8/8/1R2r3/8/8/1r2R3/8/8 b - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("e6".to_string());

        // Up 1
        let fen = String::from("8/4r3/1R6/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // Up 2
        let fen = String::from("4r3/8/1R6/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // Down 1
        let fen = String::from("8/8/1R6/4r3/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);

        // Down 2
        let fen = String::from("8/8/1R6/8/4r3/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,22);
        assert_eq!(child.capture,0);

        // Down 3
        let fen = String::from("8/8/1R6/8/8/1r2r3/8/8 w - - 0 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,1);

        // Left 1
        let fen = String::from("8/8/1R1r4/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);

        // Left 2
        let fen = String::from("8/8/1Rr5/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,0);

        // Left 3
        let fen = String::from("8/8/1r6/8/8/1r2R3/8/8 w - - 0 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // Right 1
        let fen = String::from("8/8/1R3r2/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,0);

        // Right 2
        let fen = String::from("8/8/1R4r1/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,0);

        // Right 3
        let fen = String::from("8/8/1R5r/8/8/1r2R3/8/8 w - - 1 2");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("e6".to_string()));
        assert_eq!(r.move_id,43);
        assert_eq!(child.capture,0);

        // Try (and fail with) Right 4
        let retval = r.generate_next_rook_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0x0000000000000000);
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn white_rook_moves_invisible_piece() {
        let mut r = prep_board("3kr1nR/8/8/8/8/8/8/4K3 w - - 1 2");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("h8".to_string());
        r.in_check = 1;

        // No rook moves possible
        let retval = r.generate_next_rook_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
        assert_eq!(r.move_id,10);
    }

}

