pub mod init;

// Board-to-Bit (and Square) Numbering:
// 64 63 62 61 60 59 58 57 
// 56 55 54 53 52 51 50 49
// 48 47 46 45 44 43 42 41
// 40 39 38 37 36 35 34 33
// 32 31 30 29 28 27 26 25
// 24 23 22 21 20 19 18 17
// 16 15 14 13 12 11 10 09
// 08 07 06 05 04 03 02 01

pub struct Reset {
    b_all: u64,
    b_white: u64,
    b_black: u64,
    b_pawns: u64,
    b_knights: u64,
    b_bishops: u64,
    b_rooks: u64,
    b_queens: u64,
    b_kings: u64,
    material: i8,
    moves_since_capture: u8,
    white_king_square: u8,
    black_king_square: u8,
    white_castle_q: u8,
    white_castle_k: u8,
    black_castle_q: u8,
    black_castle_k: u8
}

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
        moves_since_capture: 0,
        white_king_square: 0,
        black_king_square: 0,
        white_castle_q: 0,
        white_castle_k: 0,
        black_castle_q: 0,
        black_castle_k: 0
    };
    reset
}


impl Reset {
    pub fn print(&self) -> String {
        let mut reset_text: String = "a".to_owned();
        let appender: &str = "b";
        reset_text.push_str(appender);
        println!("{}",reset_text);
        reset_text
    }
}


