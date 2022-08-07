use crate::reset::Reset;

impl Reset {

    /// Generate the next possible bishop move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_bishop_move(&mut self, child: &mut Reset) -> bool {
        use crate::reset::r#const::B_NOT_UR_EDGE;
        use crate::reset::r#const::B_NOT_DR_EDGE;
        use crate::reset::r#const::B_NOT_DL_EDGE;
        use crate::reset::r#const::B_NOT_UL_EDGE;

        let b_available_moves: u64 = if self.white_to_move() {
            !self.b_white
        } else {
            !self.b_black
        };

        // Up Right
        let next_line = 20;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << ((self.move_id % 10) * 7);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_UR_EDGE == 0 {
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
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    return true;
                }
            }
        }

        // Down Right
        let next_line = 30;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id % 10) * 9);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_DR_EDGE == 0 {
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
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    return true;
                }
            }
        }

        // Down Left
        let next_line = 40;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id % 10) * 7);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move any farther, give up on this line
                if b_target & B_NOT_DL_EDGE == 0 {
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
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = next_line;
                    }
                    return true;
                }
            }
        }

        // Up Left
        let mut b_target = self.b_current_piece << ((self.move_id % 10) * 9);
        loop {
            println!("Move ID == {}",self.move_id);
            // If we can't move any farther, give up on this line
            if b_target & B_NOT_UL_EDGE == 0 {
                break;
            }
            b_target <<= 9;
            self.move_id += 1;
            // If my color is on the target, give up on this line
            if b_available_moves & b_target == 0 {
                break;
            }
            if self.add_move_if_valid(child, b_target) {
                // If this is a capture, we're done with this line
                if b_target & self.b_all != 0 {
                    self.consider_next_moveable_piece();
                }
                return true;
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
        r.initialize_move_generation();
        r
    }

    #[test]
    fn white_bishop_moves() {
        let mut r = prep_board("B1k5/1b6/8/8/8/K7/1B6/b7 w - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("b2".to_string());

        // Up Right 1
        let fen = String::from("B1k5/1b6/8/8/8/K1B5/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,11);
        assert_eq!(child.capture,0);

        // Up Right 2
        let fen = String::from("B1k5/1b6/8/8/3B4/K7/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // Up Right 3
        let fen = String::from("B1k5/1b6/8/4B3/8/K7/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,13);
        assert_eq!(child.capture,0);

        // Up Right 4
        let fen = String::from("B1k5/1b6/5B2/8/8/K7/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,14);
        assert_eq!(child.capture,0);

        // Up Right 5
        let fen = String::from("B1k5/1b4B1/8/8/8/K7/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,15);
        assert_eq!(child.capture,0);

        // Up Right 6
        let fen = String::from("B1k4B/1b6/8/8/8/K7/8/b7 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,16);
        assert_eq!(child.capture,0);

        // Down Right 1
        let fen = String::from("B1k5/1b6/8/8/8/K7/8/b1B5 b - - 1 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);

        // Down Left 1
        let fen = String::from("B1k5/1b6/8/8/8/K7/8/B7 b - - 0 1");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b2".to_string()));
        assert_eq!(r.move_id,40);
        assert_eq!(child.capture,1);

        // Try (and fail with) Up Left 1
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(!retval);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("a3".to_string()));
        assert_eq!(r.move_id,10);
    }

    #[test]
    fn black_bishop_moves() {
        let mut r = prep_board("B1k5/1b6/8/8/8/K7/1B6/b7 b - - 0 1");
        let mut child = reset::new();
        r.b_current_piece = utils::convert_square_to_bitstring("b7".to_string());

        // Down Right 1
        let fen = String::from("B1k5/8/2b5/8/8/K7/1B6/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);

        // Down Right 2
        let fen = String::from("B1k5/8/8/3b4/8/K7/1B6/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,22);
        assert_eq!(child.capture,0);

        // Down Right 3
        let fen = String::from("B1k5/8/8/8/4b3/K7/1B6/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,23);
        assert_eq!(child.capture,0);

        // Down Right 4
        let fen = String::from("B1k5/8/8/8/8/K4b2/1B6/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,24);
        assert_eq!(child.capture,0);

        // Down Right 5
        let fen = String::from("B1k5/8/8/8/8/K7/1B4b1/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,25);
        assert_eq!(child.capture,0);

        // Down Right 6
        let fen = String::from("B1k5/8/8/8/8/K7/1B6/b6b w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,26);
        assert_eq!(child.capture,0);

        // Down Left 1
        let fen = String::from("B1k5/8/b7/8/8/K7/1B6/b7 w - - 1 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b7".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);

        // Up Left 1
        let fen = String::from("b1k5/8/8/8/8/K7/1B6/b7 w - - 0 2");
        let retval = r.generate_next_bishop_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("c8".to_string()));
        assert_eq!(r.move_id,10);
        assert_eq!(child.capture,1);
    }
}


