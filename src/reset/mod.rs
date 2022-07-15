pub mod fen;
pub mod print;

// Board-to-Bit (and Square) Numbering:
// 64 63 62 61 60 59 58 57    a8 b8 c8 d8 e8 f8 g8 h8
// 56 55 54 53 52 51 50 49    a7 b7 c7 d7 e7 f7 g7 h7
// 48 47 46 45 44 43 42 41    a6 b6 c6 d6 e6 f6 g6 h6
// 40 39 38 37 36 35 34 33    a5 b5 c5 d5 e5 f5 g5 h5
// 32 31 30 29 28 27 26 25    a4 b4 c4 d4 e4 f4 g4 h4
// 24 23 22 21 20 19 18 17    a3 b3 c3 d3 e3 f3 g3 h3
// 16 15 14 13 12 11 10 09    a2 b2 c2 d2 e2 f2 g2 h2
// 08 07 06 05 04 03 02 01    a1 b1 c1 d1 e1 f1 g1 h1

pub struct Reset {
    //Fields passed from parent to child
    b_all: u64,                 // 8 bytes (  8)
    b_white: u64,               // 8 bytes ( 16)
    b_black: u64,               // 8 bytes ( 24)
    b_pawns: u64,               // 8 bytes ( 32)
    b_knights: u64,             // 8 bytes ( 40)
    b_bishops: u64,             // 8 bytes ( 48)
    b_rooks: u64,               // 8 bytes ( 56)
    b_queens: u64,              // 8 bytes ( 64)
    b_kings: u64,               // 8 bytes ( 72)
    material: i8,               // 1 byte  ( 73)
    halfmove_clock: u8,         // 1 byte  ( 74)
    white_king_square: u8,      // 1 byte  ( 75)
    black_king_square: u8,      // 1 byte  ( 76)
    white_castle_q: u8,         // 1 byte  ( 77) bit
    white_castle_k: u8,         // 1 byte  ( 78) bit
    black_castle_q: u8,         // 1 byte  ( 79) bit
    black_castle_k: u8,         // 1 byte  ( 80) bit

    //Fields cleared in a new child
    b_current_piece: u64,       // 8 bytes (  8)
    b_en_passant: u64,          // 8 bytes ( 16)
    b_move_data: u64,           // 8 bytes ( 24)
    score: i32,                 // 4 bytes ( 28)
    move_number: u8,            // 1 byte  ( 29)
    current_piece: u8,          // 1 byte  ( 30)
    move_data: u8,              // 1 byte  ( 31)
    capture: u8,                // 1 byte  ( 32) bit
    in_check: u8,               // 1 byte  ( 33) bit
    to_move: u8,                // 1 byte  ( 34) bit
    ep_capture: u8,             // 1 byte  ( 35) bit
    promotion: u8,              // 1 byte  ( 36) bit
    king_castled: u8,           // 1 byte  ( 37) bit
    game_over: u8,              // 1 byte  ( 38) bit

    //Fields that can be garbage in a new child
    b_from: u64,                // 8 bytes (  8)
    b_to: u64,                  // 8 bytes ( 16)
    hash_value: u32,            // 4 bytes ( 20)
    min: i32,                   // 4 bytes ( 24)
    max: i32,                   // 4 bytes ( 28)
    score_depth: u8,            // 1 bytes ( 29)
    hash_count: u8,             // 1 bytes ( 30)
    times_seen: u8,             // 1 bytes ( 31)
    from: u8,                   // 1 bytes ( 32)
    to: u8,                     // 1 bytes ( 33)
    must_check_safety: u8,      // 1 bytes ( 34) bit
}

/// Constructs a new Reset
/// 
/// # Examples
/// 
/// ```
/// # use chessica::reset::Reset;
/// let mut r = chessica::reset::new();
/// ```
pub fn new() -> Reset {
    let reset = Reset {
        b_all: 0,
        b_white: 0,
        b_black: 0,
        b_pawns: 0,
        b_knights: 0,
        b_bishops: 0,
        b_rooks: 0,
        b_queens: 0,
        b_kings: 0,
        material: 0,
        halfmove_clock: 0,
        white_king_square: 0,
        black_king_square: 0,
        white_castle_q: 0,
        white_castle_k: 0,
        black_castle_q: 0,
        black_castle_k: 0,

        b_current_piece: 0,
        b_en_passant: 0,
        b_move_data: 0,
        score: 0,
        move_number: 0,
        current_piece: 0,
        move_data: 0,
        capture: 0,
        in_check: 0,
        to_move: 0,
        ep_capture: 0,
        promotion: 0,
        king_castled: 0,
        game_over: 0,

        b_from: 0,
        b_to: 0,
        hash_value: 0,
        min: 0,
        max: 0,
        score_depth: 0,
        hash_count: 0,
        times_seen: 0,
        from: 0,
        to: 0,
        must_check_safety: 0,
    };
    reset
}

impl Reset {
    /// Initialize a child of this Reset
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use chessica::reset::Reset;
    /// let mut r = chessica::reset::new();
    /// let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 2";
    /// r.init_from_fen(fen1.to_string());
    /// let mut child = chessica::reset::new();
    /// r.init_child(&mut child);
    /// let fen = child.to_fen();
    /// assert_eq!(fen,fen2.to_string().to_string());
    /// ```
    pub fn init_child(&self, child: &mut Reset) {
        child.b_all = self.b_all;
        child.b_white = self.b_white;
        child.b_black = self.b_black;
        child.b_pawns = self.b_pawns;
        child.b_knights = self.b_knights;
        child.b_bishops = self.b_bishops;
        child.b_rooks = self.b_rooks;
        child.b_queens = self.b_queens;
        child.b_kings = self.b_kings;
        child.material = self.material;
        child.halfmove_clock = self.halfmove_clock + 1;
        child.white_king_square = self.white_king_square;
        child.black_king_square = self.black_king_square;
        child.white_castle_q = self.white_castle_q;
        child.white_castle_k = self.white_castle_k;
        child.black_castle_q = self.black_castle_q;
        child.black_castle_k = self.black_castle_k;

        child.b_current_piece = 0;
        child.b_en_passant = 0;
        child.b_move_data = 0;
        child.score = 0;
        child.move_number = self.move_number + 1;
        child.current_piece = 0;
        child.move_data = 0;
        child.capture = 0;
        child.in_check = 0;
        if self.to_move == 0 {
            child.to_move = 1;
        } else {
            child.to_move = 0;
        }
        child.ep_capture = 0;
        child.promotion = 0;
        child.king_castled = 0;
        child.game_over = 0;
    }

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
        clone.b_black = self.b_black;
        clone.b_pawns = self.b_pawns;
        clone.b_knights = self.b_knights;
        clone.b_bishops = self.b_bishops;
        clone.b_rooks = self.b_rooks;
        clone.b_queens = self.b_queens;
        clone.b_kings = self.b_kings;
        clone.material = self.material;
        clone.halfmove_clock = self.halfmove_clock;
        clone.white_king_square = self.white_king_square;
        clone.black_king_square = self.black_king_square;
        clone.white_castle_q = self.white_castle_q;
        clone.white_castle_k = self.white_castle_k;
        clone.black_castle_q = self.black_castle_q;
        clone.black_castle_k = self.black_castle_k;

        clone.b_current_piece = self.b_current_piece;
        clone.b_en_passant = self.b_en_passant;
        clone.b_move_data = self.b_move_data;
        clone.score = self.score;
        clone.move_number = self.move_number;
        clone.current_piece = self.current_piece;
        clone.move_data = self.move_data;
        clone.capture = self.capture;
        clone.in_check = self.in_check;
        clone.to_move = self.to_move;
        clone.ep_capture = self.ep_capture;
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
        clone.from = self.from;
        clone.to = self.to;
        clone.must_check_safety = self.must_check_safety;
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    #[test]
    fn reset_init_child_fen() {
        let mut r = reset::new();
        let fen1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 2";
        r.init_from_fen(fen1.to_string());
        let mut child = reset::new();
        r.init_child(&mut child);
        let result = child.to_fen();
        assert_eq!(result,fen2.to_string().to_string());
    }

    #[test]
    fn reset_init_child_fields() {
        let mut r = reset::new();
        let mut child = reset::new();
        //Fields passed from parent to child
        r.b_all = 123;
        r.b_white = 234;
        r.b_black = 456;
        r.b_pawns = 1001;
        r.b_knights = 1002;
        r.b_bishops = 1003;
        r.b_rooks = 1004;
        r.b_queens = 1005;
        r.b_kings = 1006;
        r.material = 42;
        r.halfmove_clock = 11;
        r.white_king_square = 2;
        r.black_king_square = 62;
        r.white_castle_q = 1;
        r.white_castle_k = 1;
        r.black_castle_q = 1;
        r.black_castle_k = 1;
        r.to_move = 0;
        r.move_number = 30;
        r.b_current_piece = 111;
        r.b_en_passant = 222;
        r.b_move_data = 333;
        r.score = 44;
        r.current_piece = 55;
        r.move_data = 66;
        r.capture = 1;
        r.in_check = 1;
        r.ep_capture = 1;
        r.promotion = 1;
        r.king_castled = 1;
        r.game_over = 1;
        r.init_child(&mut child);
        assert_eq!(child.b_all,123);
        assert_eq!(child.b_white,234);
        assert_eq!(child.b_black,456);
        assert_eq!(child.b_pawns,1001);
        assert_eq!(child.b_knights,1002);
        assert_eq!(child.b_bishops,1003);
        assert_eq!(child.b_rooks,1004);
        assert_eq!(child.b_queens,1005);
        assert_eq!(child.b_kings,1006);
        assert_eq!(child.material,42);
        assert_eq!(child.halfmove_clock,12); //Note the incremented value
        assert_eq!(child.white_king_square,2);
        assert_eq!(child.black_king_square,62);
        assert_eq!(child.white_castle_q,1);
        assert_eq!(child.white_castle_k,1);
        assert_eq!(child.black_castle_q,1);
        assert_eq!(child.black_castle_k,1);
        assert_eq!(child.to_move,1); //Note the change
        assert_eq!(child.move_number,31); //Note the incremented value
        assert_eq!(child.b_current_piece,0); // Cleared
        assert_eq!(child.b_en_passant,0); // Cleared
        assert_eq!(child.b_move_data,0); // Cleared
        assert_eq!(child.score,0); // Cleared
        assert_eq!(child.current_piece,0); // Cleared
        assert_eq!(child.move_data,0); // Cleared
        assert_eq!(child.capture,0); // Cleared
        assert_eq!(child.in_check,0); // Cleared
        assert_eq!(child.ep_capture,0); // Cleared
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

    #[test]
    fn reset_clone_to_fields() {
        let mut r = reset::new();
        let mut child = reset::new();
        //Fields passed from parent to child
        r.b_all = 123;
        r.b_white = 234;
        r.b_black = 456;
        r.b_pawns = 1001;
        r.b_knights = 1002;
        r.b_bishops = 1003;
        r.b_rooks = 1004;
        r.b_queens = 1005;
        r.b_kings = 1006;
        r.material = 42;
        r.halfmove_clock = 11;
        r.white_king_square = 2;
        r.black_king_square = 62;
        r.white_castle_q = 1;
        r.white_castle_k = 1;
        r.black_castle_q = 1;
        r.black_castle_k = 1;
        r.to_move = 0;
        r.move_number = 30;
        r.b_current_piece = 111;
        r.b_en_passant = 222;
        r.b_move_data = 333;
        r.score = 44;
        r.current_piece = 55;
        r.move_data = 66;
        r.capture = 1;
        r.in_check = 1;
        r.ep_capture = 1;
        r.promotion = 1;
        r.king_castled = 1;
        r.game_over = 1;
        r.clone_to(&mut child);
        assert_eq!(child.b_all,123);
        assert_eq!(child.b_white,234);
        assert_eq!(child.b_black,456);
        assert_eq!(child.b_pawns,1001);
        assert_eq!(child.b_knights,1002);
        assert_eq!(child.b_bishops,1003);
        assert_eq!(child.b_rooks,1004);
        assert_eq!(child.b_queens,1005);
        assert_eq!(child.b_kings,1006);
        assert_eq!(child.material,42);
        assert_eq!(child.halfmove_clock,11);
        assert_eq!(child.white_king_square,2);
        assert_eq!(child.black_king_square,62);
        assert_eq!(child.white_castle_q,1);
        assert_eq!(child.white_castle_k,1);
        assert_eq!(child.black_castle_q,1);
        assert_eq!(child.black_castle_k,1);
        assert_eq!(child.to_move,0);
        assert_eq!(child.move_number,30);
        assert_eq!(child.b_current_piece,111);
        assert_eq!(child.b_en_passant,222);
        assert_eq!(child.b_move_data,333);
        assert_eq!(child.score,44);
        assert_eq!(child.current_piece,55);
        assert_eq!(child.move_data,66);
        assert_eq!(child.capture,1);
        assert_eq!(child.in_check,1);
        assert_eq!(child.ep_capture,1);
        assert_eq!(child.promotion,1);
        assert_eq!(child.king_castled,1);
        assert_eq!(child.game_over,1);
    }

    #[test]
    fn reset_clone_to() {
    }
}
