use crate::reset::Reset;

impl Reset {
    pub fn init_from_fen(&mut self, fen: String) {
        println!("{}",fen);
        println!("{}",self.b_all);
        let chunks:Vec<&str>= fen.split(" ").collect();
        println!("chunk 0: {}",chunks[0]);
        println!("chunk 1: {}",chunks[1]);
        println!("chunk 2: {}",chunks[2]);
        println!("chunk 3: {}",chunks[3]);
        println!("chunk 4: {}",chunks[4]);
        println!("chunk 5: {}",chunks[5]);

        // PROCESS THE PIECE POSITIONS (Chunk 0)
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
                        bit = bit << 7 - x + 8*(7 - y as u32);
                        self.b_all |= bit;
                        match c {
                            'k'|'q'|'r'|'b'|'n'|'p' => {
                                self.b_black |= bit;
                            },
                            _ => {
                                self.b_white |= bit;
                            },
                        }
                        match c {
                            'k'|'K' => {
                                self.b_kings |= bit;
                            },
                            'q'|'Q' => {
                                self.b_queens |= bit;
                            },
                            'r'|'R' => {
                                self.b_rooks |= bit;
                            },
                            'b'|'B' => {
                                self.b_bishops |= bit;
                            },
                            'n'|'N' => {
                                self.b_knights |= bit;
                            },
                            _ => {
                                self.b_pawns |= bit;
                            },
                        }
                        x += 1;
                    },
                    _ => println!("I don't know what to do with {}",c),
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn init_reset_from_fen_starting_position() {
        use crate::reset;
        let mut r = reset::new();
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        r.init_from_fen(starting_fen);
        assert_eq!(r.b_all,0xffff00000000ffff,"b_all");
        assert_eq!(r.b_white,0x000000000000ffff,"b_white");
        assert_eq!(r.b_black,0xffff000000000000,"b_black");
        assert_eq!(r.b_pawns,0x00ff00000000ff00,"b_pawns");
        assert_eq!(r.b_knights,0x4200000000000042,"b_knights");
        assert_eq!(r.b_bishops,0x2400000000000024,"b_bishops");
        assert_eq!(r.b_rooks,0x8100000000000081,"b_rooks");
        assert_eq!(r.b_queens,0x1000000000000010,"b_queens");
        assert_eq!(r.b_kings,0x0800000000000008,"b_kings");
    }
}
