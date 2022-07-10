pub mod init;

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

pub mod sub1 {
    pub fn hello() {
        println!("Hello again world!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2,4);
    }
}
