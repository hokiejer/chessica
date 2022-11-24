use crate::reset::Reset;
use crate::reset::PieceType;

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
        clone.b_kings = self.b_kings;
        clone.reserved_01 = self.reserved_01;
        clone.material = self.material;
        clone.halfmove_clock = self.halfmove_clock;
        clone.fullmove_number = self.fullmove_number;
        clone.white_king_square = self.white_king_square;
        clone.black_king_square = self.black_king_square;
        clone.castle_bits = self.castle_bits;
        clone.reserved_02 = self.reserved_02;
        clone.reserved_03 = self.reserved_03;
        clone.reserved_04 = self.reserved_04;
        clone.reserved_05 = self.reserved_05;
        clone.reserved_06 = self.reserved_06;
        clone.reserved_07 = self.reserved_07;
        clone.reserved_08 = self.reserved_08;
        clone.reserved_09 = self.reserved_09;
        clone.reserved_10 = self.reserved_10;
        clone.reserved_11 = self.reserved_11;

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
        clone.bi_from = self.bi_from;
        clone.bi_to = self.bi_to;
        clone.score_depth = self.score_depth;
        clone.hash_count = self.hash_count;
        clone.times_seen = self.times_seen;
        clone.must_check_safety = self.must_check_safety;
        clone.bi_current_piece = self.bi_current_piece;
        clone.pin_dimension = self.pin_dimension;
        clone.current_piece_type = self.current_piece_type;
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::PieceType;
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
        r.b_kings = 1006;
        r.material = 42;
        r.halfmove_clock = 11;
        r.fullmove_number = 15;
        r.white_king_square = 2;
        r.black_king_square = 62;
        r.castle_bits = 99;
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
        r.must_check_safety = 1;
        r.bi_current_piece = 132;
        r.pin_dimension = 14;
        r.current_piece_type = PieceType::Queen;
        r.clone_to(&mut child);
        assert_eq!(child.b_all,123);
        assert_eq!(child.b_white,234);
        assert_eq!(child.b_pawns,1001);
        assert_eq!(child.b_knights,1002);
        assert_eq!(child.b_bishops,1003);
        assert_eq!(child.b_rooks,1004);
        assert_eq!(child.b_kings,1006);
        assert_eq!(child.material,42);
        assert_eq!(child.halfmove_clock,11);
        assert_eq!(child.fullmove_number,15);
        assert_eq!(child.white_king_square,2);
        assert_eq!(child.black_king_square,62);
        assert_eq!(child.castle_bits,99);
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
        assert_eq!(child.must_check_safety,1);
        assert_eq!(child.bi_current_piece,132);
        assert_eq!(child.pin_dimension,14);
        assert_eq!(child.current_piece_type,PieceType::Queen);
    }

    #[test]
    fn reset_clone_to() {
    }
}

