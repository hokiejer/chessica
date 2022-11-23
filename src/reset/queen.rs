use crate::reset::Reset;
use crate::reset::pinned::PIN_MATCH_NS;
use crate::reset::pinned::PIN_MATCH_EW;
use crate::reset::pinned::PIN_MATCH_NESW;
use crate::reset::pinned::PIN_MATCH_SENW;

impl Reset {
    /// Generate the next possible queen move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_queen_move(&mut self, child: &mut Reset) -> bool {
        use crate::reset::r#const::B_NOT_N_EDGE;
        use crate::reset::r#const::B_NOT_E_EDGE;
        use crate::reset::r#const::B_NOT_W_EDGE;
        use crate::reset::r#const::B_NOT_S_EDGE;
        use crate::reset::r#const::B_NOT_NE_EDGE;
        use crate::reset::r#const::B_NOT_SE_EDGE;
        use crate::reset::r#const::B_NOT_SW_EDGE;
        use crate::reset::r#const::B_NOT_NW_EDGE;

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

        // Northeast
        let next_line = 30;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << ((self.move_id % 10) * 7);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_NE_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target <<= 7;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_NESW) {
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
        let next_line = 40;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> (self.move_id % 10);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_E_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target >>= 1;
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

        // Southeast
        let next_line = 50;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id % 10) * 9);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_SE_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target >>= 9;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_SENW) {
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
        let next_line = 60;
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

        // Southwest
        let next_line = 70;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id % 10) * 7);
            loop {
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_SW_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target >>= 7;
                self.move_id += 1;
                // If my color is on the target, give up on this line
                if b_available_moves & b_target == 0 {
                    self.move_id = next_line;
                    break;
                }
                if self.add_move_if_valid(child, b_target, PIN_MATCH_NESW) {
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
        let next_line = 80;
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

        // Northwest
        let mut b_target = self.b_current_piece << ((self.move_id % 10) * 9);
        loop {
            // If we can't move any farther, give up on this line
            if b_target & B_NOT_NW_EDGE == 0 {
                break;
            }
            b_target <<= 9;
            self.move_id += 1;
            // If my color is on the target, give up on this line
            if b_available_moves & b_target == 0 {
                break;
            }
            if self.add_move_if_valid(child, b_target, PIN_MATCH_SENW) {
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
    fn white_queen_moves() {
        let mut r = prep_board("4k3/2P5/8/3pnr1p/8/1PQ2q2/8/4K2R w - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("c3");

        // Up 1
        let fen = String::from("4k3/2P5/8/3pnr1p/2Q5/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up 2
        let fen = String::from("4k3/2P5/8/2Qpnr1p/8/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up 3
        let fen = String::from("4k3/2P5/2Q5/3pnr1p/8/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Up Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/3Q4/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up Right 2
        let fen = String::from("4k3/2P5/8/3pQr1p/8/1P3q2/8/4K2R b - - 0 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,1);

        // Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P1Q1q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Right 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P2Qq2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Right 3
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3Q2/8/4K2R b - - 0 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,0);

        // Down Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3q2/3Q4/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3q2/2Q5/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,51);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3q2/8/2Q1K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,52);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down Left 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3q2/1Q6/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,61);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down Left 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1P3q2/8/Q3K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,62);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up Left 1
        let fen = String::from("4k3/2P5/8/3pnr1p/1Q6/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,81);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up Left 2
        let fen = String::from("4k3/2P5/8/Q2pnr1p/8/1P3q2/8/4K2R b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c3".to_string()));
        assert_eq!(r.move_id,82);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Try (and fail with) Up Left 3
        let retval = r.generate_next_queen_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b3".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn black_queen_moves() {
        let mut r = prep_board("4k3/2P5/8/3pnr1p/8/1PQ2q2/8/4K2R b - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("f3");

        // Up 1
        let fen = String::from("4k3/2P5/8/3pnr1p/5q2/1PQ5/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Up Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/6q1/1PQ5/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ3q1/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Right 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ4q/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,32);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down Right 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/6q1/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Down Right 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/8/4K2q w - - 0 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,50);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,1);

        // Down 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/5q2/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,51);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Down 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/8/4Kq1R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,52);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Down Left 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/4q3/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,61);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Down Left 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ5/8/3qK2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,62);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Left 1
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQ1q3/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,71);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Left 2
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1PQq4/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,72);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);

        // Left 3
        let fen = String::from("4k3/2P5/8/3pnr1p/8/1Pq5/8/4K2R w - - 0 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,80);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,1);

        // Up Left 1
        let fen = String::from("4k3/2P5/8/3pnr1p/4q3/1PQ5/8/4K2R w - - 1 2");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f3".to_string()));
        assert_eq!(r.move_id,81);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,1);

        // Try (and fail with) Up Left 2
        let retval = r.generate_next_queen_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("h5".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn white_queen_moves_block_check() {
        let mut r = prep_board("3k4/3r4/5PP1/2P2QP1/5PP1/8/2P5/3K4 w - - 0 1");
        let mut child = reset::new();
        r.current_piece_init("f5");

        // Down Left 2
        let fen = String::from("3k4/3r4/5PP1/2P3P1/5PP1/3Q4/2P5/3K4 b - - 1 1");
        let retval = r.generate_next_queen_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("f5".to_string()));
        assert_eq!(r.move_id,62);
        assert_eq!(child.capture,0);
        assert_eq!(child.in_check,0);
    }

    #[test]
    fn black_queen_moves_invisible_piece() {
        let mut r = prep_board("3k4/8/8/4N3/4Pq2/4p3/8/3RK3 b - - 1 2");
        let mut child = reset::new();
        r.current_piece_init("f4");
        r.in_check = 1;

        // No queen moves possible
        let retval = r.generate_next_queen_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("d8".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn white_queen_moves_invisible_piece() {
        let mut r = prep_board("3kr1nQ/6p1/8/8/8/8/8/4K3 w - - 1 2");
        let mut child = reset::new();
        r.current_piece_init("h8");
        r.in_check = 1;

        // No queen moves possible
        let retval = r.generate_next_queen_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,0);
    }

}


