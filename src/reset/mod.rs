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
    let r = Reset {
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
    r
}

pub fn from_fen(fen: String) -> Reset {
    let mut r = Reset {
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
    println!("{}",fen);
    let chunks:Vec<&str>= fen.split(" ").collect();
    println!("chunk 0: {}",chunks[0]);
    println!("chunk 1: {}",chunks[1]);
    println!("chunk 2: {}",chunks[2]);
    println!("chunk 3: {}",chunks[3]);
    println!("chunk 4: {}",chunks[4]);
    println!("chunk 5: {}",chunks[5]);
    let rows:Vec<&str>= chunks[0].split("/").collect();
    for y in 0..8 {
        let mut x = 0;
        for c in rows[y].chars() {
            let mut bit: u64 = 1; 
            match c {
                '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => {
                    x += c as u32 - '0' as u32;
                },
                'k'|'q'|'r'|'b'|'n'|'p'|'K'|'Q'|'R'|'B'|'N'|'P' => {
                    bit = bit << x + 8*(7 - y as u32);
                    r.b_all &= bit;
                    match c {
                        'k'|'q'|'r'|'b'|'n'|'p' => {
                            r.b_black &= bit;
                        },
                        _ => {
                            r.b_white &= bit;
                        },
                    }
                    match c {
                        'k'|'K' => {
                            r.b_kings &= bit;
                        },
                        'q'|'Q' => {
                            r.b_queens &= bit;
                        },
                        'r'|'R' => {
                            r.b_rooks &= bit;
                        },
                        'b'|'B' => {
                            r.b_bishops &= bit;
                        },
                        'n'|'N' => {
                            r.b_knights &= bit;
                        },
                        _ => {
                            r.b_pawns &= bit;
                        },
                    }
                    x += 1;
                },
                _ => println!("I don't know what to do with {}",c),
            }
        }
    }
    r
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
