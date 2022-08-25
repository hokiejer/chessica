use crate::reset::Reset;

impl Reset {

    /// Clone this Reset
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use chessica::reset::Reset;
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// let mut child = chessica::reset::new();
    /// r.clone_to(&mut child);
    /// let result = child.to_fen();
    /// assert_eq!(result,fen.to_string().to_string());
    /// ```
    pub fn clone_to(&self, clone: &mut Reset) {
        clone.b_all = self.b_all;
        clone.b_white = self.b_white;
        clone.b_pawns = self.b_pawns;
        clone.b_knights = self.b_knights;
        clone.b_bishops = self.b_bishops;
        clone.b_rooks = self.b_rooks;
        clone.b_queens = self.b_queens;
        clone.b_kings = self.b_kings;
        clone.material = self.material;
        clone.halfmove_clock = self.halfmove_clock;
        clone.fullmove_number = self.fullmove_number;
        clone.white_king_square = self.white_king_square;
        clone.black_king_square = self.black_king_square;
        clone.white_castle_q = self.white_castle_q;
        clone.white_castle_k = self.white_castle_k;
        clone.black_castle_q = self.black_castle_q;
        clone.black_castle_k = self.black_castle_k;

        clone.b_current_piece = self.b_current_piece;
        clone.b_en_passant = self.b_en_passant;
        clone.score = self.score;
        clone.move_id = self.move_id;
        clone.to_move = self.to_move;
        clone.capture = self.capture;
        clone.in_check = self.in_check;
        clone.promotion = self.promotion;
        clone.king_castled = self.king_castled;
        clone.game_over = self.game_over;

        clone.b_from = self.b_from;
        clone.b_to = self.b_to;
        clone.hash_value = self.hash_value;
        clone.min = self.min;
        clone.max = self.max;
        clone.score_depth = self.score_depth;
        clone.hash_count = self.hash_count;
        clone.times_seen = self.times_seen;
        clone.must_check_safety = self.must_check_safety;
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
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

    #[test]
    fn reset_clone_to_fields() {
        let mut r = reset::new();
        let mut child = reset::new();
        //Fields passed from parent to child
        r.b_all = 123;
        r.b_white = 234;
        r.b_pawns = 1001;
        r.b_knights = 1002;
        r.b_bishops = 1003;
        r.b_rooks = 1004;
        r.b_queens = 1005;
        r.b_kings = 1006;
        r.material = 42;
        r.halfmove_clock = 11;
        r.fullmove_number = 15;
        r.white_king_square = 2;
        r.black_king_square = 62;
        r.white_castle_q = 1;
        r.white_castle_k = 1;
        r.black_castle_q = 1;
        r.black_castle_k = 1;
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
        r.clone_to(&mut child);
        assert_eq!(child.b_all,123);
        assert_eq!(child.b_white,234);
        assert_eq!(child.b_pawns,1001);
        assert_eq!(child.b_knights,1002);
        assert_eq!(child.b_bishops,1003);
        assert_eq!(child.b_rooks,1004);
        assert_eq!(child.b_queens,1005);
        assert_eq!(child.b_kings,1006);
        assert_eq!(child.material,42);
        assert_eq!(child.halfmove_clock,11);
        assert_eq!(child.fullmove_number,15);
        assert_eq!(child.white_king_square,2);
        assert_eq!(child.black_king_square,62);
        assert_eq!(child.white_castle_q,1);
        assert_eq!(child.white_castle_k,1);
        assert_eq!(child.black_castle_q,1);
        assert_eq!(child.black_castle_k,1);
        assert_eq!(child.to_move,0);
        assert_eq!(child.move_id,30);
        assert_eq!(child.b_current_piece,111);
        assert_eq!(child.b_en_passant,222);
        assert_eq!(child.score,44);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,1);
        assert_eq!(child.promotion,1);
        assert_eq!(child.king_castled,1);
        assert_eq!(child.game_over,1);
    }

    #[test]
    fn reset_clone_to() {
    }
}

