use crate::reset::Reset;

impl Reset {

    /// Generate the next possible rook move
    ///
    /// Returns `true` if a move was suggested, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn generate_next_rook_move(&mut self, child: &mut Reset) -> bool {
        use crate::reset::r#const::B_NOT_TOP_EDGE;
        use crate::reset::r#const::B_NOT_RIGHT_EDGE;
        use crate::reset::r#const::B_NOT_LEFT_EDGE;
        use crate::reset::r#const::B_NOT_BOTTOM_EDGE;

        let b_available_moves: u64 = if self.white_to_move() {
            !self.b_white
        } else {
            !self.b_black
        };

        // Up
        let next_line = 20;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << ((self.move_id - 10) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_TOP_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target <<= 8;
                self.move_id += 1;
                // If my color is on the target, we're done with this line
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

        // Down
        let next_line = 30;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece >> ((self.move_id - 20) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_BOTTOM_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target >>= 8;
                self.move_id += 1;
                // If my color is on the target, we're done with this line
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

        // Left
        let next_line = 40;
        if self.move_id < next_line {
            let mut b_target = self.b_current_piece << (self.move_id - 30);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move left anymore, give up
                if b_target & B_NOT_LEFT_EDGE == 0 {
                    self.move_id = next_line;
                    break;
                }
                b_target <<= 1;
                self.move_id += 1;
                // If my color is on the target, we're done with this line
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

        // Right
        let mut b_target = self.b_current_piece >> ((self.move_id - 40));
        loop {
            println!("Move ID == {}",self.move_id);
            // If we can't move right anymore, give up
            if b_target & B_NOT_RIGHT_EDGE == 0 {
                break;
            }
            b_target >>= 1;
            self.move_id += 1;
            // If my color is on the target, we're done with this line
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

        println!("I got here!");
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
    fn white_rook_moves_no_kings() {
        let mut r = prep_board("8/8/1R2r3/8/8/1r2R3/8/8 w - - 0 1");
        let mut child = reset::new();
        r.init_child(&mut child);
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
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,12);
        assert_eq!(child.capture,0);

        // Down 1
        let fen = String::from("8/8/4r3/1R6/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,21);
        assert_eq!(child.capture,0);

        // Down 2
        let fen = String::from("8/8/4r3/8/1R6/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,22);
        assert_eq!(child.capture,0);

        // Down 3
        let fen = String::from("8/8/4r3/8/8/1R2R3/8/8 b - - 0 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,1);

        // Left 1
        let fen = String::from("8/8/R3r3/8/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,31);
        assert_eq!(child.capture,0);

        // Right 1
        let fen = String::from("8/8/2R1r3/8/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,41);
        assert_eq!(child.capture,0);

        // Right 2
        let fen = String::from("8/8/3Rr3/8/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,42);
        assert_eq!(child.capture,0);

        // Right 3
        let fen = String::from("8/8/4R3/8/8/1r2R3/8/8 b - - 0 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,0x0000000000000000);
        assert_eq!(r.move_id,10);
        assert_eq!(child.capture,1);

    }
}

