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
/// Resets contain the following fields:
///
/// ## Fields passed from parent to child
///
/// | field             | type | size | total | description |
/// | ----------------- | ---- | ---- | ----- | ----------- |
/// | b_all             | u64  | 8    |    8  | Bitstring representing the presence of any piece |
/// | b_white           | u64  | 8    |   16  | Bitstring representing the presence of white pieces |
/// | b_black           | u64  | 8    |   24  | Bitstring representing the presence of black pieces |
/// | b_pawns           | u64  | 8    |   32  | Bitstring representing the presence of pawns |
/// | b_knights         | u64  | 8    |   40  | Bitstring representing the presence of knights |
/// | b_bishops         | u64  | 8    |   48  | Bitstring representing the presence of bishops |
/// | b_rooks           | u64  | 8    |   56  | Bitstring representing the presence of rooks |
/// | b_queens          | u64  | 8    |   64  | Bitstring representing the presence of queens |
/// | b_kings           | u64  | 8    |   72  | Bitstring representing the presence of kings |
/// | material          | i8   | 1    |   73  | Material score of this board |
/// | halfmove_clock    | u8   | 1    |   74  | Halfmoves elapsed since last pawn move or capture |
/// | fullmove_number   | u8   | 1    |   75  | Full moves elapsed since beginning of the game |
/// | white_king_square | u8   | 1    |   76  | Square number of the white king |
/// | black_king_square | u8   | 1    |   77  | Square number of the black king |
/// | white_castle_q    | u8   | 1    |   78  | `1` if white is eligible to castle queenside, `0` if not |
/// | white_castle_k    | u8   | 1    |   79  | `1` if white is eligible to castle kingside, `0` if not |
/// | black_castle_q    | u8   | 1    |   80  | `1` if black is eligible to castle queenside, `0` if not |
/// | black_castle_k    | u8   | 1    |   81  | `1` if black is eligible to castle kingside, `0` if not |
///
/// ## Fields cleared in a new child
///
/// | field             | type | size | total | description |
/// | ----------------- | ---- | ---- | ----- | ----------- |
/// | b_current_piece   | u64  | 8    |    8  | Bitstring representing the piece currently under consideration for move generation |
/// | b_en_passant      | u64  | 8    |   16  | Bitstring representing a piece that is eligible for en passant capture.  This is an entire bitstring to represent a single bit, which seems wasteful. |
/// | b_move_data       | u64  | 8    |   24  | Is this even needed anymore? |
/// | score             | i32  | 4    |   28  | Score of this reset.  White is positive, Black negative.  If white is up exactly a pawn, the score will be 1,000,000.  Checkmate for Black is -128,000,000. |
/// | move_id           | u8   | 1    |   29  | ID of tne next move to be considered for a given piece type |
/// | to_move           | u8   | 1    |   30  | `0` if it is white's move, `1` if it is black's move |
/// | move_data         | u8   | 1    |   31  | Is this even needed anymore? |
/// | capture           | u8   | 1    |   32  | `1` if the last move was a capture, `0` otherwise |
/// | in_check          | u8   | 1    |   33  | `1` if the side moving is currently in check, `0` otherwise |
/// | ep_capture        | u8   | 1    |   34  | `1` if the last move was an en passant capture, `0` otherwise |
/// | promotion         | u8   | 1    |   35  | `1` if the last move was a promotion, `0` otherwise |
/// | king_castled      | u8   | 1    |   36  | `1` if the last move was a castle, `0` otherwise |
/// | game_over         | u8   | 1    |   37  | `1` if the game is over |
///
///
/// ## Fields that can be garbage in a new child
///
/// | field             | type | size | total | description |
/// | ----------------- | ---- | ---- | ----- | ----------- |
/// | b_from            | u64  | 8    |    8  | Bitstring representing where the last piece was moved from |
/// | b_to              | u64  | 8    |   16  | Bitstring representing where the last piece was moved to |
/// | hash_value        | u32  | 4    |   20  | Hash value for this reset |
/// | min               | i32  | 8    |   24  | Min value used for move searching |
/// | max               | i32  | 8    |   28  | Max value used for move searching |
/// | score_depth       | u8   | 1    |   29  | Search depth from which score was obtained |
/// | hash_count        | u8   | 1    |   30  | Number of times this reset was saved to the hash table |
/// | times_seen        | u8   | 1    |   31  | Number of times this reset has been seen in the current game |
/// | must_check_safety | u8   | 1    |   32  | 1 if we must check king safety after this move, 0 otherwise.  I believe this is used for odd moves, like EP captures, castling, and promotions. |
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
    fullmove_number: u8,        // 1 byte  ( 74)
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
    move_id: u8,                // 1 byte  ( 29)
    to_move: u8,                // 1 byte  ( 30) bit
    move_data: u8,              // 1 byte  ( 31)
    capture: u8,                // 1 byte  ( 32) bit
    in_check: u8,               // 1 byte  ( 33) bit
    ep_capture: u8,             // 1 byte  ( 34) bit
    promotion: u8,              // 1 byte  ( 35) bit
    king_castled: u8,           // 1 byte  ( 36) bit
    game_over: u8,              // 1 byte  ( 37) bit

    //Fields that can be garbage in a new child
    b_from: u64,                // 8 bytes (  8)
    b_to: u64,                  // 8 bytes ( 16)
    hash_value: u32,            // 4 bytes ( 20)
    min: i32,                   // 4 bytes ( 24)
    max: i32,                   // 4 bytes ( 28)
    score_depth: u8,            // 1 bytes ( 29)
    hash_count: u8,             // 1 bytes ( 30)
    times_seen: u8,             // 1 bytes ( 31)
    must_check_safety: u8,      // 1 bytes ( 32) bit
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
        b_black: 0,
        b_pawns: 0,
        b_knights: 0,
        b_bishops: 0,
        b_rooks: 0,
        b_queens: 0,
        b_kings: 0,
        material: 0,
        halfmove_clock: 0,
        fullmove_number: 0,
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
        move_id: 0,
        to_move: 0,
        move_data: 0,
        capture: 0,
        in_check: 0,
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
        must_check_safety: 0,
    }
}

