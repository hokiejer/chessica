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
    if king == revealed {
        return RevealedCheckSearchType::DoNotSearch;
    }

    if king < revealed { // Attack from W, NW, N, or NE
        let difference: u8 = revealed - king;
        if difference % 8 == 0 {
            return RevealedCheckSearchType::FromN;
        } else if difference % 9 == 0 {
            if (revealed - 1) % 8 > (king - 1) % 8 {
                return RevealedCheckSearchType::FromNW;
            }
        } else if difference % 7 == 0 {
            if (revealed - 1) % 8 < (king - 1) % 8 {
                return RevealedCheckSearchType::FromNE;
            }
        } else if revealed <= king - (king % 8) + 8 {
            return RevealedCheckSearchType::FromW;
        }
    } else { // Attack from E, SE, S, or SW
        let difference: u8 = king - revealed;
        if difference % 8 == 0 {
            return RevealedCheckSearchType::FromS;
        } else if difference % 9 == 0 {
            if (revealed - 1) % 8 < (king - 1) % 8 {
                return RevealedCheckSearchType::FromSE;
            }
        } else if difference % 7 == 0 {
            if (revealed - 1) % 8 > (king - 1) % 8 {
                return RevealedCheckSearchType::FromSW;
            }
        } else if revealed > king - (king % 8) {
            return RevealedCheckSearchType::FromE;
        }
    }
    RevealedCheckSearchType::DoNotSearch
}

lazy_static! {
    static ref REVEALED_CHECK_ROUTES: Vec<Vec<RevealedCheckSearchType>> = {
        let mut vec: Vec<Vec<RevealedCheckSearchType>> = Vec::new();

        let mut blank: Vec<RevealedCheckSearchType> = Vec::new();
        vec.push(blank); // push a blank at index 0

        for king in 1..65 { // indexes 1 to 64
            let mut mini_router: Vec<RevealedCheckSearchType> = Vec::new();
            mini_router.push(RevealedCheckSearchType::DoNotSearch); // push a blank at index 0
            for revealed in 1..65 { // indexes 1 to 64
                mini_router.push(revealed_check_router(king as u8,revealed as u8));
            }
            vec.push(mini_router);
        }
        vec
    };
}

pub fn revealed_check_bitmap(king: u8, search: RevealedCheckSearchType) -> u64 {
    use crate::reset::r#const::B_NOT_UL_EDGE;
    use crate::reset::r#const::B_NOT_UR_EDGE;
    use crate::reset::r#const::B_NOT_DL_EDGE;
    use crate::reset::r#const::B_NOT_DR_EDGE;
    use crate::reset::r#const::B_NOT_TOP_EDGE;
    use crate::reset::r#const::B_NOT_RIGHT_EDGE;
    use crate::reset::r#const::B_NOT_LEFT_EDGE;
    use crate::reset::r#const::B_NOT_BOTTOM_EDGE;

    let b_king: u64 = 0x0000000000000001 << (king - 1);
    let mut b_temp: u64 = b_king;
    let mut b_map: u64 = 0x0000000000000000;
    match search {
        RevealedCheckSearchType::FromN => {
            // Go N from the king
            while b_temp & B_NOT_TOP_EDGE != 0 {
                b_temp <<= 8;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNE => {
            // Go NE from the king
            while b_temp & B_NOT_UR_EDGE != 0 {
                b_temp <<= 7;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromE => {
            // Go E from the king
            while b_temp & B_NOT_RIGHT_EDGE != 0 {
                b_temp >>= 1;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSE => {
            // Go SE from the king
            while b_temp & B_NOT_DR_EDGE != 0 {
                b_temp >>= 9;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromS => {
            // Go S from the king
            while b_temp & B_NOT_BOTTOM_EDGE != 0 {
                b_temp >>= 8;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSW => {
            // Go SW from the king
            while b_temp & B_NOT_DL_EDGE != 0 {
                b_temp >>= 7;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromW => {
            // Go W from the king
            while b_temp & B_NOT_LEFT_EDGE != 0 {
                b_temp <<= 1;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNW => {
            // Go NW from the king
            while b_temp & B_NOT_UL_EDGE != 0 {
                b_temp <<= 9;
                b_map |= b_temp;
            }
        },
        _ => {},
    }
    b_map
}

//lazy_static! {
//    static ref REVEALED_CHECK_BITMAPS: Vec<Array<u64>> = {
//        Vec::new()
//    }
//}

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
    fn revealed_check_routes() {
        // Trusts revealed_check_router, this just ensures that the matrix matches the function
        use crate::reset::safe_revealed::revealed_check_router;
        use crate::reset::safe_revealed::RevealedCheckSearchType;
        use crate::reset::safe_revealed::REVEALED_CHECK_ROUTES;
        for king in 1..65 {
            for revealed in 1..65 {
                println!("king == {}, revealed = {}",king,revealed);
                assert_eq!(REVEALED_CHECK_ROUTES[king][revealed],revealed_check_router(king as u8,revealed as u8));
            }
        }
    }

    #[test]
    fn revealed_check_bitmap_1() {
        use crate::reset::safe_revealed::RevealedCheckSearchType;
        use crate::reset::safe_revealed::revealed_check_bitmap;
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromN),0x0101010101000000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromNE),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromE),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromSE),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromS),0x0000000000000101);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromSW),0x0000000000000204);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromW),0x0000000000fe0000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::FromNW),0x2010080402000000);
        assert_eq!(revealed_check_bitmap(17,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);

        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromN),0x8080808080000000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromNE),0x0408102040000000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromE),0x00000000007f0000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromSE),0x0000000000004020);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromS),0x0000000000008080);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromSW),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromW),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::FromNW),0x0000000000000000);
        assert_eq!(revealed_check_bitmap(24,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);

        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromN),0x1010100000000000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromNE),0x0204080000000000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromE),0x0000000f00000000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromSE),0x0000000008040201);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromS),0x0000000010101010);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromSW),0x0000000020408000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromW),0x000000e000000000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::FromNW),0x8040200000000000);
        assert_eq!(revealed_check_bitmap(37,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);
    }

    #[test]
    fn revealed_check_from_n() {
        let mut r = prep_board("2K5/4Q3/8/8/2n5/8/8/4k3 w - - 0 1");
        r.b_from = utils::convert_square_to_bitstring("e3".to_string());
        r.b_to = utils::convert_square_to_bitstring("c4".to_string());
        assert!(!r.black_is_safe_from_revealed_check());
    }
}
