use crate::reset::Reset;

impl Reset {
    /// Initialize a Reset from FEN notation
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut r = reset::new();
    /// let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// r.init_from_fen(fen);
    /// ```
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
                        let mut material_multiplier: i8 = 0;
                        match c {
                            'k'|'q'|'r'|'b'|'n'|'p' => {
                                self.b_black |= bit;
                                material_multiplier = -1;
                            },
                            _ => {
                                self.b_white |= bit;
                                material_multiplier = 1;
                            },
                        }
                        match c {
                            'k'|'K' => {
                                self.b_kings |= bit;
                            },
                            'q'|'Q' => {
                                self.b_queens |= bit;
                                self.material += material_multiplier * 9;
                            },
                            'r'|'R' => {
                                self.b_rooks |= bit;
                                self.material += material_multiplier * 5;
                            },
                            'b'|'B' => {
                                self.b_bishops |= bit;
                                self.material += material_multiplier * 3;
                            },
                            'n'|'N' => {
                                self.b_knights |= bit;
                                self.material += material_multiplier * 3;
                            },
                            _ => {
                                self.b_pawns |= bit;
                                self.material += material_multiplier * 1;
                            },
                        }
                        x += 1;
                    },
                    _ => println!("I don't know what to do with {}",c),
                }
            }
        }
        
        // PROCESS WHO'S MOVE IT IS (Chunk 1)
        match chunks[1] {
            "b" => {
                self.to_move = 1;
            },
            "w" => {
                self.to_move = 0;
            },
            _ => println!("I don't know what to do with {}",chunks[1]),
        }

        // PROCESS CASTLE ELIGIBILITY (Chunk 2)
        for c in chunks[2].chars() {
            match c {
                '-' => {},
                'K' => {
                    self.white_castle_k = 1;
                },
                'Q' => {
                    self.white_castle_q = 1;
                },
                'k' => {
                    self.black_castle_k = 1;
                },
                'q' => {
                    self.black_castle_q = 1;
                },
                _ => println!("I don't know what to do with {}",c),
            }
        }

        // PROCESS EN PASSANT SQUARE (Chunk 3)
        if chunks[3] != "-" {
            use crate::utils;
            self.b_en_passant = utils::convert_square_to_bitstring(chunks[3].to_string());
        }

        // PROCESS HALFMOVE CLOCK (Chunk 4)
        self.halfmove_clock = chunks[4].parse().unwrap();

        // PROCESS HALFMOVE CLOCK (Chunk 4)
        self.move_number = chunks[5].parse().unwrap();
    }

    /// Generate a FEN notation string from a reset
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut r = reset::new();
    /// let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// r.init_from_fen(fen);
    /// let my_fen = r.get_fen();
    /// ```
    pub fn get_fen(&mut self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    #[test]
    fn fen_init_from_fen_starting_position() {
        let mut r = reset::new();
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let fen_copy = fen.clone();
        r.init_from_fen(fen);
        assert_eq!(r.b_all,0xffff00000000ffff,"b_all");
        assert_eq!(r.b_white,0x000000000000ffff,"b_white");
        assert_eq!(r.b_black,0xffff000000000000,"b_black");
        assert_eq!(r.b_pawns,0x00ff00000000ff00,"b_pawns");
        assert_eq!(r.b_knights,0x4200000000000042,"b_knights");
        assert_eq!(r.b_bishops,0x2400000000000024,"b_bishops");
        assert_eq!(r.b_rooks,0x8100000000000081,"b_rooks");
        assert_eq!(r.b_queens,0x1000000000000010,"b_queens");
        assert_eq!(r.b_kings,0x0800000000000008,"b_kings");
        assert_eq!(r.material,0,"material");
        assert_eq!(r.to_move,0,"to_move");
        assert_eq!(r.white_castle_k,1,"white_castle_k");
        assert_eq!(r.white_castle_q,1,"white_castle_q");
        assert_eq!(r.black_castle_k,1,"black_castle_k");
        assert_eq!(r.black_castle_q,1,"black_castle_q");
        assert_eq!(r.b_en_passant,0,"b_en_passant");
        assert_eq!(r.halfmove_clock,0,"halfmove_clock");
        assert_eq!(r.move_number,1,"move_number");
        //let generated_fen = r.get_fen();
        //assert_eq!(generated_fen,fen_copy,"FEN generation");
    }

    #[test]
    fn fen_init_from_fen_botvinnik_capablanca() {
        let mut r = reset::new();
        let fen = String::from("r3r1k1/p2q1ppp/1pn2n2/3p4/P1pP4/2P1P3/1BQ1NPPP/4RRK1 w - - 4 17");
        let fen_copy = fen.clone();
        r.init_from_fen(fen);
        assert_eq!(r.b_all,0x8a976410b0286f0e,"b_all");
        assert_eq!(r.b_white,0x0000000090286f0e,"b_white");
        assert_eq!(r.b_black,0x8a97641020000000,"b_black");
        assert_eq!(r.b_pawns,0x00874010b0280700,"b_pawns");
        assert_eq!(r.b_knights,0x0000240000000800,"b_knights");
        assert_eq!(r.b_bishops,0x0000000000004000,"b_bishops");
        assert_eq!(r.b_rooks,0x880000000000000c,"b_rooks");
        assert_eq!(r.b_queens,0x0010000000002000,"b_queens");
        assert_eq!(r.b_kings,0x0200000000000002,"b_kings");
        assert_eq!(r.material,0,"material");
        assert_eq!(r.to_move,0,"to_move");
        assert_eq!(r.white_castle_k,0,"white_castle_k");
        assert_eq!(r.white_castle_q,0,"white_castle_q");
        assert_eq!(r.black_castle_k,0,"black_castle_k");
        assert_eq!(r.black_castle_q,0,"black_castle_q");
        assert_eq!(r.b_en_passant,0,"b_en_passant");
        assert_eq!(r.halfmove_clock,4,"halfmove_clock");
        assert_eq!(r.move_number,17,"move_number");
        //let generated_fen = r.get_fen();
        //assert_eq!(generated_fen,fen_copy,"FEN generation");
    }

    #[test]
    fn fen_init_from_fen_jibberish_01() {
        let mut r = reset::new();
        let fen = String::from("rk6/pn1qPp1q/np2P3/4P1p1/P1p1p2r/R3P1bP/NBQ3P1/6K1 b - - 4 17");
        let fen_copy = fen.clone();
        r.init_from_fen(fen);
        assert_eq!(r.b_all,0xc0ddc80aa98be202,"b_all");
        assert_eq!(r.b_white,0x000808088089e202,"b_white");
        assert_eq!(r.b_black,0xc0d5c00229020000,"b_black");
        assert_eq!(r.b_pawns,0x008c480aa8090200,"b_pawns");
        assert_eq!(r.b_knights,0x0040800000008000,"b_knights");
        assert_eq!(r.b_bishops,0x0000000000024000,"b_bishops");
        assert_eq!(r.b_rooks,0x8000000001800000,"b_rooks");
        assert_eq!(r.b_queens,0x0011000000002000,"b_queens");
        assert_eq!(r.b_kings,0x4000000000000002,"b_kings");
        assert_eq!(r.material,-16,"material");
        assert_eq!(r.to_move,1,"to_move");
        assert_eq!(r.white_castle_k,0,"white_castle_k");
        assert_eq!(r.white_castle_q,0,"white_castle_q");
        assert_eq!(r.black_castle_k,0,"black_castle_k");
        assert_eq!(r.black_castle_q,0,"black_castle_q");
        assert_eq!(r.b_en_passant,0,"b_en_passant");
        assert_eq!(r.halfmove_clock,4,"halfmove_clock");
        assert_eq!(r.move_number,17,"move_number");
        //let generated_fen = r.get_fen();
        //assert_eq!(generated_fen,fen_copy,"FEN generation");
    }

    #[test]
    fn fen_init_from_fen_en_passant() {
        let mut r = reset::new();
        let fen = String::from("rnbqkbnr/ppppp1pp/8/4P3/5pP1/8/PPPP1P1P/RNBQKBNR b KQkq g3 0 1");
        let fen_copy = fen.clone();
        r.init_from_fen(fen);
        assert_eq!(r.b_all,0xfffb00080600f5ff,"b_all");
        assert_eq!(r.b_white,0x000000080200f5ff,"b_white");
        assert_eq!(r.b_black,0xfffb000004000000,"b_black");
        assert_eq!(r.b_pawns,0x00fb00080600f500,"b_pawns");
        assert_eq!(r.b_knights,0x4200000000000042,"b_knights");
        assert_eq!(r.b_bishops,0x2400000000000024,"b_bishops");
        assert_eq!(r.b_rooks,0x8100000000000081,"b_rooks");
        assert_eq!(r.b_queens,0x1000000000000010,"b_queens");
        assert_eq!(r.b_kings,0x0800000000000008,"b_kings");
        assert_eq!(r.material,0,"material");
        assert_eq!(r.to_move,1,"to_move");
        assert_eq!(r.white_castle_k,1,"white_castle_k");
        assert_eq!(r.white_castle_q,1,"white_castle_q");
        assert_eq!(r.black_castle_k,1,"black_castle_k");
        assert_eq!(r.black_castle_q,1,"black_castle_q");
        assert_eq!(r.b_en_passant,0x0000000000020000,"b_en_passant");
        assert_eq!(r.halfmove_clock,0,"halfmove_clock");
        assert_eq!(r.move_number,1,"move_number");
        //let generated_fen = r.get_fen();
        //assert_eq!(generated_fen,fen_copy,"FEN generation");
    }
}
