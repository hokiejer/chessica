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
        if self.move_id < 20 {
            let mut b_target = self.b_current_piece << ((self.move_id - 10) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_TOP_EDGE == 0 {
                    self.move_id = 20;
                    break;
                }
                b_target <<= 8;
                self.move_id += 1;
                if b_available_moves & b_target == 0 {
                    self.move_id = 20;
                    break;
                }
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = 20;
                    }
                    return true;
                }
            }
        }

        // Down
        if self.move_id < 30 {
            let mut b_target = self.b_current_piece >> ((self.move_id - 20) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_BOTTOM_EDGE == 0 {
                    self.move_id = 30;
                    break;
                }
                b_target >>= 8;
                self.move_id += 1;
                if b_available_moves & b_target == 0 {
                    self.move_id = 30;
                    break;
                }
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = 30;
                    }
                    return true;
                }
            }
        }

        // Left
        if self.move_id < 40 {
            let mut b_target = self.b_current_piece << ((self.move_id - 30) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_TOP_EDGE == 0 {
                    self.move_id = 40;
                    break;
                }
                b_target <<= 8;
                self.move_id += 1;
                if b_available_moves & b_target == 0 {
                    self.move_id = 40;
                    break;
                }
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = 40;
                    }
                    return true;
                }
            }
        }

        // Right
        if self.move_id < 50 {
            let mut b_target = self.b_current_piece << ((self.move_id - 40) * 8);
            loop {
                println!("Move ID == {}",self.move_id);
                // If we can't move up anymore, give up
                if b_target & B_NOT_TOP_EDGE == 0 {
                    self.move_id = 50;
                    break;
                }
                b_target <<= 8;
                self.move_id += 1;
                if b_available_moves & b_target == 0 {
                    self.move_id = 50;
                    break;
                }
                if self.add_move_if_valid(child, b_target) {
                    // If this is a capture, we're done with this line
                    if b_target & self.b_all != 0 {
                        self.move_id = 50;
                    }
                    return true;
                }
            }
        }

        println!("I got here!");
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
    fn white_rook_moves_no_kings() {
        let mut r = prep_board("8/8/1R2r3/8/8/1r2R3/8/8 w - - 0 1");
        let mut child = reset::new();
        r.init_child(&mut child);
        r.b_current_piece = utils::convert_square_to_bitstring("b6".to_string());

        // First Move: Up 1
        let fen = String::from("8/1R6/4r3/8/8/1r2R3/8/8 b - - 1 1");
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,11);

        // Second Move: Up 2
        let fen = String::from("1R6/8/4r3/8/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,12);

        // Down 1
        let fen = String::from("8/8/4r3/1R6/8/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,21);

        // Down 2
        let fen = String::from("8/8/4r3/8/1R6/1r2R3/8/8 b - - 1 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,22);

        // Down 3
        let fen = String::from("8/8/4r3/8/8/1R2R3/8/8 b - - 0 1");
        r.init_child(&mut child);
        let retval = r.generate_next_rook_move(&mut child);
        assert!(retval);
        assert_eq!(child.to_fen(),fen);
        assert_eq!(r.b_current_piece,utils::convert_square_to_bitstring("b6".to_string()));
        assert_eq!(r.move_id,30);
        assert_eq!(child.capture,1);

    }
}

