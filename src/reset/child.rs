use crate::reset::Reset;
use crate::reset::pinned::PinDimension;

impl Reset {
    /// Initialize a child of this Reset
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use chessica::reset::Reset;
    /// let mut r = chessica::reset::new();
    /// let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen1.to_string());
    /// let mut child = chessica::reset::new();
    /// r.init_child(&mut child);
    /// let fen = child.to_fen();
    /// assert_eq!(fen,fen2.to_string().to_string());
    /// ```
    pub fn init_child(&self, child: &mut Reset) {
        child.b_all = self.b_all;
        child.b_white = self.b_white;
        child.b_pawns = self.b_pawns;
        child.b_knights = self.b_knights;
        child.b_bishops = self.b_bishops;
        child.b_rooks = self.b_rooks;
        child.b_kings = self.b_kings;
        child.reserved_01 = self.reserved_01;
        child.material = self.material;
        child.halfmove_clock = self.halfmove_clock;
        child.fullmove_number = self.fullmove_number;
        child.white_king_square = self.white_king_square;
        child.black_king_square = self.black_king_square;
        child.castle_bits = self.castle_bits;
        child.reserved_02 = self.reserved_02;
        child.reserved_03 = self.reserved_03;
        child.reserved_04 = self.reserved_04;
        child.reserved_05 = self.reserved_05;
        child.reserved_06 = self.reserved_06;
        child.reserved_07 = self.reserved_07;
        child.reserved_08 = self.reserved_08;
        child.reserved_09 = self.reserved_09;
        child.reserved_10 = self.reserved_10;
        child.reserved_11 = self.reserved_11;

        child.b_current_piece = 0;
        child.b_en_passant = 0;
        child.score = 0;
        child.move_id = 0;
        child.to_move = 0;
        child.capture = 0;
        child.in_check = 0;
        child.promotion = 0;
        child.king_castled = 0;
        child.game_over = 0;

        child.hash_value = 0;
        child.min = 0;
        child.max = 0;
        child.bi_from = 0;
        child.bi_to = 0;
        child.score_depth = 0;
        child.hash_count = 0;
        child.times_seen = 0;
        child.must_check_safety = 0;
        child.bi_current_piece = 0;
        child.pin_dimension = PinDimension::Unset;
        child.reserved_14 = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    #[test]
    fn reset_init_child_fen() {
        let mut r = reset::new();
        // Note that the halfmove_clock, fullmove_number, and to_move change elsewhere
        let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen3 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen1.to_string());
        let mut child = reset::new();
        r.init_child(&mut child);
        let result = child.to_fen();
        assert_eq!(result,fen2.to_string(),"child");
        let mut grandchild = reset::new();
        child.init_child(&mut grandchild);
        let result = grandchild.to_fen();
        assert_eq!(result,fen3.to_string(),"grandchild");
    }

    #[test]
    fn reset_init_child_fields() {
        let mut r = reset::new();
        let mut child = reset::new();
        //Fields passed from parent to child
        r.b_all = 123;
        r.b_white = 234;
        r.b_pawns = 1001;
        r.b_knights = 1002;
        r.b_bishops = 1003;
        r.b_rooks = 1004;
        r.b_kings = 1006;
        r.material = 42;
        r.halfmove_clock = 11;
        r.fullmove_number = 15;
        r.white_king_square = 2;
        r.black_king_square = 62;
        r.castle_bits = 134;
        r.to_move = 0;
        r.move_id = 30;
        r.b_current_piece = 111;
        r.b_en_passant = 222;
        r.score = 44;
        r.capture = 1;
        r.in_check = 1;
        r.promotion = 1;
        r.king_castled = 1;
        r.game_over = 1;
        r.init_child(&mut child);
        assert_eq!(child.b_all,123);
        assert_eq!(child.b_white,234);
        assert_eq!(child.b_pawns,1001);
        assert_eq!(child.b_knights,1002);
        assert_eq!(child.b_bishops,1003);
        assert_eq!(child.b_rooks,1004);
        assert_eq!(child.b_kings,1006);
        assert_eq!(child.material,42);
        assert_eq!(child.halfmove_clock,11); //Note that we increment later
        assert_eq!(child.fullmove_number,15); //No change
        assert_eq!(child.white_king_square,2);
        assert_eq!(child.black_king_square,62);
        assert_eq!(child.castle_bits,134);
        assert_eq!(child.to_move,0); //Note that we change this later
        assert_eq!(child.move_id,0); // Cleared
        assert_eq!(child.b_current_piece,0); // Cleared
        assert_eq!(child.b_en_passant,0); // Cleared
        assert_eq!(child.score,0); // Cleared
        assert_eq!(child.capture,0); // Cleared
        assert_eq!(child.in_check,0); // Cleared
        assert_eq!(child.promotion,0); // Cleared
        assert_eq!(child.king_castled,0); // Cleared
        assert_eq!(child.game_over,0); // Cleared
    }

    #[test]
    fn reset_clone_to_fen() {
        let mut r = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        let mut child = reset::new();
        r.clone_to(&mut child);
        let result = child.to_fen();
        assert_eq!(result,fen.to_string().to_string());
    }
}

