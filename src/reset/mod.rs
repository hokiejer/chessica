pub mod r#const;
pub mod fen;
pub mod print;
pub mod child;
pub mod clone;
pub mod helpers;
pub mod capture;
pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;
pub mod moves;
pub mod safe;
pub mod safe_revealed;
pub mod safe_direct;
pub mod profiling;

/// The complete status of a chess game at a given time
///
/// # Board Mappings
///
/// A Reset maps the squares of the chessboard as follows:
/// ```text
/// // "Number"                // "Square"               // Bitstring
/// 64 63 62 61 60 59 58 57    a8 b8 c8 d8 e8 f8 g8 h8   0x0000000000000000
/// 56 55 54 53 52 51 50 49    a7 b7 c7 d7 e7 f7 g7 h7     ^              ^
/// 48 47 46 45 44 43 42 41    a6 b6 c6 d6 e6 f6 g6 h6   64-61           4-1
/// 40 39 38 37 36 35 34 33    a5 b5 c5 d5 e5 f5 g5 h5
/// 32 31 30 29 28 27 26 25    a4 b4 c4 d4 e4 f4 g4 h4
/// 24 23 22 21 20 19 18 17    a3 b3 c3 d3 e3 f3 g3 h3
/// 16 15 14 13 12 11 10 09    a2 b2 c2 d2 e2 f2 g2 h2
/// 08 07 06 05 04 03 02 01    a1 b1 c1 d1 e1 f1 g1 h1
/// ```
///
/// # Reset Fields
///
/// Note that 128-bit blocks need to be homogeneous in order to optimize Reset child
/// initialization.
/// 
/// Resets contain the following fields:
///
/// ## Fields passed from parent to child
///
/// | field             | type | child? | offset | description |
/// | ----------------- | ---- | ------ | ----- | ----------- |
/// | b_all             | u64  | copy   |    8  | Bitstring representing the presence of any piece |
/// | b_white           | u64  | copy   |   16  | Bitstring representing the presence of white pieces. |
/// |                   |      |        |       | Note that there is no `b_black` - a user must call `b_black()` to derive this value. |
/// | b_pawns           | u64  | copy   |   24  | Bitstring representing the presence of pawns |
/// | b_knights         | u64  | copy   |   32  | Bitstring representing the presence of knights |
/// | b_bishops         | u64  | copy   |   40  | Bitstring representing the presence of bishops |
/// | b_rooks           | u64  | copy   |   48  | Bitstring representing the presence of rooks |
/// |                   |      |        |       | Note that there is no `b_queens` - a user must call `b_queens()` to derive this value. |
/// | b_kings           | u64  | copy   |   56  | Bitstring representing the presence of kings |
/// | reserved_01       | u64  | copy   |   64  | Reserved |
/// | material          | i8   | copy   |   65  | Material score of this board |
/// | halfmove_clock    | u8   | copy   |   66  | Halfmoves elapsed since last pawn move or capture |
/// | fullmove_number   | u8   | copy   |   67  | Full moves elapsed since beginning of the game |
/// | white_king_square | u8   | copy   |   68  | Square number of the white king |
/// | black_king_square | u8   | copy   |   69  | Square number of the black king |
/// | castle_bits       | u8   | copy   |   70  | `1` if white is eligible to castle queenside, `0` if not |
/// | white_castle_k    |      |        |       | 0x01: `1` if white is eligible to castle kingside, `0` if not |
/// | white_castle_q    |      |        |       | 0x02: `1` if white is eligible to castle queenside, `0` if not |
/// | black_castle_k    |      |        |       | 0x04: `1` if black is eligible to castle kingside, `0` if not |
/// | black_castle_q    |      |        |       | 0x08: `1` if black is eligible to castle queenside, `0` if not |
/// | reserved_02       | u8   | copy   |   71  | Reserved |
/// | reserved_03       | u8   | copy   |   72  | Reserved |
/// | reserved_04       | u8   | copy   |   73  | Reserved |
/// | reserved_05       | u8   | copy   |   74  | Reserved |
/// | reserved_06       | u8   | copy   |   75  | Reserved |
/// | reserved_07       | u8   | copy   |   76  | Reserved |
/// | reserved_08       | u8   | copy   |   77  | Reserved |
/// | reserved_09       | u8   | copy   |   78  | Reserved |
/// | reserved_10       | u8   | copy   |   79  | Reserved |
/// | reserved_11       | u8   | copy   |   80  | Reserved |
///
/// ## Fields cleared in a new child
///
/// | field             | type | child? | total | description |
/// | ----------------- | ---- | ------ | ----- | ----------- |
/// | b_current_piece   | u64  | clear  |    8  | Bitstring representing the piece currently under consideration for move generation |
/// | b_en_passant      | u64  | clear  |   16  | Bitstring representing a piece that is eligible for en passant capture.  This is an entire bitstring to represent a single bit, which seems wasteful. |
/// | score             | i32  | clear  |   20  | Score of this reset.  White is positive, Black negative.  If white is up exactly a pawn, the score will be 1,000,000.  Checkmate for Black is -128,000,000. |
/// | move_id           | u8   | clear  |   21  | ID of tne next move to be considered for a given piece type |
/// | to_move           | u8   | clear  |   22  | `0` if it is white's move, `1` if it is black's move |
/// | capture           | u8   | clear  |   23  | `1` if the last move was a capture, `0` otherwise |
/// | in_check          | u8   | clear  |   24  | `1` if the side moving is currently in check, `0` otherwise |
/// | promotion         | u8   | clear  |   25  | `1` if the last move was a promotion, `0` otherwise |
/// | king_castled      | u8   | clear  |   26  | `1` if the last move was a castle, `0` otherwise |
/// | game_over         | u8   | clear  |   27  | `1` if the game is over |
///
///
/// ## Fields that can be garbage in a new child
///
/// | field             | type | child? | total | description |
/// | ----------------- | ---- | ------ | ----- | ----------- |
/// | b_from            | u64  | whatev |    8  | Bitstring representing where the last piece was moved from |
/// | b_to              | u64  | whatev |   16  | Bitstring representing where the last piece was moved to |
/// | hash_value        | u32  | whatev |   20  | Hash value for this reset |
/// | min               | i32  | whatev |   24  | Min value used for move searching |
/// | max               | i32  | whatev |   28  | Max value used for move searching |
/// | bi_from           | u8   | whatev |   29  | Bit index of the move's originating square |
/// | bi_to             | u8   | whatev |   29  | Bit index of the move's destination square |
/// | score_depth       | u8   | whatev |   29  | Search depth from which score was obtained |
/// | hash_count        | u8   | whatev |   30  | Number of times this reset was saved to the hash table |
/// | times_seen        | u8   | whatev |   31  | Number of times this reset has been seen in the current game |
/// | must_check_safety | u8   | whatev |   32  | 1 if we must check king safety after this move, 0 otherwise.  I believe this is used for odd moves, like EP captures, castling, and promotions. |
/// | reserved_12       | u8   | whatev |   32  | Reserved |
/// | reserved_13       | u8   | whatev |   32  | Reserved |
/// | reserved_14       | u8   | whatev |   32  | Reserved |
pub struct Reset {
    //Fields passed from parent to child
    b_all: u64,                 // 8 bytes (  8)
    b_white: u64,               // 8 bytes ( 16)
    b_pawns: u64,               // 8 bytes ( 24)
    b_knights: u64,             // 8 bytes ( 32)
    b_bishops: u64,             // 8 bytes ( 40)
    b_rooks: u64,               // 8 bytes ( 48)
    b_kings: u64,               // 8 bytes ( 56)
    reserved_01: u64,               // 8 bytes ( 56)
    material: i8,               // 1 byte  ( 57)
    halfmove_clock: u8,         // 1 byte  ( 58)
    fullmove_number: u8,        // 1 byte  ( 59)
    white_king_square: u8,      // 1 byte  ( 60)
    black_king_square: u8,      // 1 byte  ( 61)
    castle_bits: u8,            // 1 byte  ( 62) bit
    reserved_02: u8,            // 1 byte  ( 62) bit
    reserved_03: u8,            // 1 byte  ( 62) bit
    reserved_04: u8,            // 1 byte  ( 62) bit
    reserved_05: u8,            // 1 byte  ( 62) bit
    reserved_06: u8,            // 1 byte  ( 62) bit
    reserved_07: u8,            // 1 byte  ( 62) bit
    reserved_08: u8,            // 1 byte  ( 62) bit
    reserved_09: u8,            // 1 byte  ( 62) bit
    reserved_10: u8,            // 1 byte  ( 62) bit
    reserved_11: u8,            // 1 byte  ( 62) bit

    //Fields cleared in a new child
    b_current_piece: u64,       // 8 bytes (  8)
    b_en_passant: u64,          // 8 bytes ( 16)
    score: i32,                 // 4 bytes ( 20)
    move_id: u8,                // 1 byte  ( 21)
    to_move: u8,                // 1 byte  ( 22) bit
    capture: u8,                // 1 byte  ( 23) bit
    in_check: u8,               // 1 byte  ( 24) bit
    promotion: u8,              // 1 byte  ( 25) bit
    king_castled: u8,           // 1 byte  ( 26) bit
    game_over: u8,              // 1 byte  ( 27) bit

    //Fields that can be garbage in a new child
    b_from: u64,                // 8 bytes (  8)
    b_to: u64,                  // 8 bytes ( 16)
    hash_value: u32,            // 4 bytes ( 20)
    min: i32,                   // 4 bytes ( 24)
    max: i32,                   // 4 bytes ( 28)
    bi_from: u8,                // 1 bytes ( 29)
    bi_to: u8,                  // 1 bytes ( 30)
    score_depth: u8,            // 1 bytes ( 31)
    hash_count: u8,             // 1 bytes ( 32)
    times_seen: u8,             // 1 bytes ( 33)
    must_check_safety: u8,      // 1 bytes ( 34) bit
    reserved_12: u8,            // 1 byte  ( 62) bit
    reserved_13: u8,            // 1 byte  ( 62) bit
    reserved_14: u8,            // 1 byte  ( 62) bit
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
    Reset {
        b_all: 0,
        b_white: 0,
        b_pawns: 0,
        b_knights: 0,
        b_bishops: 0,
        b_rooks: 0,
        b_kings: 0,
        reserved_01: 0,
        material: 0,
        halfmove_clock: 0,
        fullmove_number: 0,
        white_king_square: 0,
        black_king_square: 0,
        castle_bits: 0,
        reserved_02: 0,
        reserved_03: 0,
        reserved_04: 0,
        reserved_05: 0,
        reserved_06: 0,
        reserved_07: 0,
        reserved_08: 0,
        reserved_09: 0,
        reserved_10: 0,
        reserved_11: 0,

        b_current_piece: 0,
        b_en_passant: 0,
        score: 0,
        move_id: 0,
        to_move: 0,
        capture: 0,
        in_check: 0,
        promotion: 0,
        king_castled: 0,
        game_over: 0,

        b_from: 0,
        b_to: 0,
        hash_value: 0,
        min: 0,
        max: 0,
        bi_from: 0,
        bi_to: 0,
        score_depth: 0,
        hash_count: 0,
        times_seen: 0,
        must_check_safety: 0,
        reserved_12: 0,
        reserved_13: 0,
        reserved_14: 0,
    }
}

/// Bitstring of all black pieces
///
/// This dynamically replaces `b_black` that used to be a Reset field
impl Reset {
    pub fn b_black(&self) -> u64 {
        self.b_all & !self.b_white
    }
}

/// Bitstring of all queen locations
///
/// This dynamically replaces `b_queens` that used to be a Reset field
impl Reset {
    pub fn b_queens(&self) -> u64 {
        self.b_all & !(self.b_pawns | self.b_knights | self.b_bishops | self.b_rooks | self.b_kings)
    }
}

/// White Castle Kingside is available?
///
/// This dynamically replaces `b_queens` that used to be a Reset field
impl Reset {
    pub fn white_castle_k(&self) -> bool {
        use crate::bitops::r#const::U8_BIT1;
        self.castle_bits & U8_BIT1 != 0
    }
}

/// White Castle Queenside is available?
///
/// This dynamically replaces `b_queens` that used to be a Reset field
impl Reset {
    pub fn white_castle_q(&self) -> bool {
        use crate::bitops::r#const::U8_BIT2;
        self.castle_bits & U8_BIT2 != 0
    }
}

/// Black Castle Kingside is available?
///
/// This dynamically replaces `b_queens` that used to be a Reset field
impl Reset {
    pub fn black_castle_k(&self) -> bool {
        use crate::bitops::r#const::U8_BIT3;
        self.castle_bits & U8_BIT3 != 0
    }
}

/// Black Castle Queenside is available?
///
/// This dynamically replaces `b_queens` that used to be a Reset field
impl Reset {
    pub fn black_castle_q(&self) -> bool {
        use crate::bitops::r#const::U8_BIT4;
        self.castle_bits & U8_BIT4 != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;

    #[test]
    fn reset_b_black() {
        let mut r = reset::new();
        r.b_all = 0xf0f0f0f0f0f0f0f0;
        r.b_white = 0xf000f000f000f000;
        assert_eq!(r.b_black(),0x00f000f000f000f0);
        r.b_all = 0x0000700000000002;
        r.b_white = 0x0000600000000000;
        assert_eq!(r.b_black(),0x0000100000000002);
    }

    #[test]
    fn reset_b_queens() {
        let mut r = reset::new();
        r.b_all = 0x99df144c840d63ec;
        r.b_pawns = 0x00c7100884096200;
        r.b_knights = 0x0010040000040040;
        r.b_bishops = 0x0008000400000024;
        r.b_rooks = 0x8100000000000180;
        r.b_kings = 0x0800000000000008;
        assert_eq!(r.b_queens(),0x1000004000000000);
    }
}

