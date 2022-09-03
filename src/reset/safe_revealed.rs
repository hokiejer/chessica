use crate::reset::Reset;

#[derive(PartialEq,Debug)]
pub enum RevealedCheckSearchType {
    DoNotSearch,
    FromN,
    FromNE,
    FromE,
    FromSE,
    FromS,
    FromSW,
    FromW,
    FromNW,
}

    pub fn revealed_check_router(king: u8, revealed: u8) -> RevealedCheckSearchType {

        println!("How Should I Search For Revealed Check: {} {}",king,revealed); 
        if king == revealed {
            println!("  This is the king's square.  Don't search.");
            return RevealedCheckSearchType::DoNotSearch;
        }

        if king < revealed { // Attack from W, NW, N, or NE
            let difference: u8 = (revealed - king);
            println!("  revealed - king == {}",revealed - king); 
            if difference % 8 == 0 {
                println!("  Attack from N");
                return RevealedCheckSearchType::FromN;
            } else if difference % 9 == 0 {
                println!("  revealed - 1 % 8 == {}",(revealed - 1) % 8);
                println!("  king - 1 % 8 == {}",(king - 1) % 8);
                if (revealed - 1) % 8 > (king - 1) % 8 {
                    println!("  Attack from NW");
                    return RevealedCheckSearchType::FromNW;
                }
            } else if difference % 7 == 0 {
                println!("  revealed - 1 % 8 == {}",(revealed - 1) % 8);
                println!("  king - 1 % 8 == {}",(king - 1) % 8);
                if (revealed - 1) % 8 < (king - 1) % 8 {
                    println!("  Attack from NE");
                    return RevealedCheckSearchType::FromNE;
                }
            } else if revealed <= king - (king % 8) + 8 {
                println!("  Attack from W");
                return RevealedCheckSearchType::FromW;
            }
        } else { // Attack from E, SE, S, or SW
            let difference: u8 = (king - revealed);
            println!("  king - revealed == {}",king - revealed); 
            if difference % 8 == 0 {
                println!("  Attack from S");
                return RevealedCheckSearchType::FromS;
            } else if difference % 9 == 0 {
                println!("  revealed - 1 % 8 == {}",(revealed - 1) % 8);
                println!("  king - 1 % 8 == {}",(king - 1) % 8);
                if (revealed - 1) % 8 < (king - 1) % 8 {
                    println!("  Attack from SE");
                    return RevealedCheckSearchType::FromSE;
                }
            } else if difference % 7 == 0 {
                println!("  revealed - 1 % 8 == {}",(revealed - 1) % 8);
                println!("  king - 1 % 8 == {}",(king - 1) % 8);
                if (revealed - 1) % 8 > (king - 1) % 8 {
                    println!("  Attack from SW");
                    return RevealedCheckSearchType::FromSW;
                }
            } else if revealed > king - (king % 8) {
                println!("  Attack from E");
                return RevealedCheckSearchType::FromE;
            }
        }
        println!("  Do Not Search");
        RevealedCheckSearchType::DoNotSearch
    }

impl Reset {

    pub fn white_is_safe_from_revealed_check(&mut self) -> bool {
        self.is_safe_from_revealed_check(1)
    }

    pub fn black_is_safe_from_revealed_check(&mut self) -> bool {
        self.is_safe_from_revealed_check(0)
    }

    pub fn is_safe_from_revealed_check(&mut self, opponent: u8) -> bool {
        let b_king: u64 = if opponent == 0 { // black king safety
            self.b_kings & self.b_black()
        } else { // White king safety
            self.b_kings & self.b_white
        };
        use crate::bitops;
        let bi_from = bitops::get_bit_number(self.b_from);
        println!("bi_from == {}",bi_from);
        let bi_king = bitops::get_bit_number(b_king);
        println!("bi_king == {}",bi_king);
        println!("bi_from - bi_king % 8 == {}",(bi_from - bi_king) % 8);
        println!("bi_from - bi_king & 0x07 == {}",(bi_from - bi_king) & 0x07);
        false
    }

        
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn revealed_check_router_1() {
        use crate::reset::safe_revealed::revealed_check_router;
        use crate::reset::safe_revealed::RevealedCheckSearchType;
        assert_eq!(revealed_check_router(20, 1),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20, 2),RevealedCheckSearchType::FromSE);
        assert_eq!(revealed_check_router(20, 3),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20, 4),RevealedCheckSearchType::FromS);
        assert_eq!(revealed_check_router(20, 5),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20, 6),RevealedCheckSearchType::FromSW);
        assert_eq!(revealed_check_router(20, 7),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20, 8),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20, 9),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,10),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,11),RevealedCheckSearchType::FromSE);
        assert_eq!(revealed_check_router(20,12),RevealedCheckSearchType::FromS);
        assert_eq!(revealed_check_router(20,13),RevealedCheckSearchType::FromSW);
        assert_eq!(revealed_check_router(20,14),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,15),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,16),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,17),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(20,18),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(20,19),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(20,20),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,21),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(20,22),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(20,23),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(20,24),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(20,25),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,26),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,27),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(20,28),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(20,29),RevealedCheckSearchType::FromNW);
        assert_eq!(revealed_check_router(20,30),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,31),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,32),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,33),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,34),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(20,35),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,36),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(20,37),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,38),RevealedCheckSearchType::FromNW);
        assert_eq!(revealed_check_router(20,39),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,40),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,41),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(20,42),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,43),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,44),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(20,45),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,46),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,47),RevealedCheckSearchType::FromNW);
        assert_eq!(revealed_check_router(20,48),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,49),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,50),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,51),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,52),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(20,53),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,54),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,55),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,56),RevealedCheckSearchType::FromNW);
        assert_eq!(revealed_check_router(20,57),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,58),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,59),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,60),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(20,61),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,62),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,63),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(20,64),RevealedCheckSearchType::DoNotSearch);
    }

    #[test]
    fn revealed_check_router_2() {
        use crate::reset::safe_revealed::revealed_check_router;
        use crate::reset::safe_revealed::RevealedCheckSearchType;
        assert_eq!(revealed_check_router(31, 1),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 2),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 3),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 4),RevealedCheckSearchType::FromSE);
        assert_eq!(revealed_check_router(31, 5),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 6),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 7),RevealedCheckSearchType::FromS);
        assert_eq!(revealed_check_router(31, 8),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31, 9),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,10),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,11),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,12),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,13),RevealedCheckSearchType::FromSE);
        assert_eq!(revealed_check_router(31,14),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,15),RevealedCheckSearchType::FromS);
        assert_eq!(revealed_check_router(31,16),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,17),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,18),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,19),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,20),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,21),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,22),RevealedCheckSearchType::FromSE);
        assert_eq!(revealed_check_router(31,23),RevealedCheckSearchType::FromS);
        assert_eq!(revealed_check_router(31,24),RevealedCheckSearchType::FromSW);
        assert_eq!(revealed_check_router(31,25),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,26),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,27),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,28),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,29),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,30),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(31,31),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,32),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(31,33),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,34),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,35),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,36),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,37),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,38),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(31,39),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(31,40),RevealedCheckSearchType::FromNW);
        assert_eq!(revealed_check_router(31,41),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,42),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,43),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,44),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,45),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(31,46),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,47),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(31,48),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,49),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,50),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,51),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,52),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(31,53),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,54),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,55),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(31,56),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,57),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,58),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,59),RevealedCheckSearchType::FromNE);
        assert_eq!(revealed_check_router(31,60),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,61),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,62),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(31,63),RevealedCheckSearchType::FromN);
        assert_eq!(revealed_check_router(31,64),RevealedCheckSearchType::DoNotSearch);
    }

    #[test]
    fn revealed_check_from_n() {
        let mut r = prep_board("2K5/4Q3/8/8/2n5/8/8/4k3 w - - 0 1");
        r.b_from = utils::convert_square_to_bitstring("e3".to_string());
        r.b_to = utils::convert_square_to_bitstring("c4".to_string());
        assert!(!r.black_is_safe_from_revealed_check());
    }
}
