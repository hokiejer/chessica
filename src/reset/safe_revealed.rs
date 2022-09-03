use crate::reset::Reset;
use enum_map::{enum_map,Enum,EnumMap};
use std::collections::HashMap;

use crate::reset::r#const::BLACK;
use crate::reset::r#const::WHITE;

#[derive(PartialEq,Eq,Hash,Debug)]
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
        } else if difference <= 8 - ((king - 1) % 8) - 1 {
            return RevealedCheckSearchType::FromW;
        } else if difference % 9 == 0 {
            if (revealed - 1) % 8 > (king - 1) % 8 {
                return RevealedCheckSearchType::FromNW;
            }
        } else if difference % 7 == 0 {
            if (revealed - 1) % 8 < (king - 1) % 8 {
                return RevealedCheckSearchType::FromNE;
            }
        }
    } else { // Attack from E, SE, S, or SW
        let difference: u8 = king - revealed;
        if difference % 8 == 0 {
            return RevealedCheckSearchType::FromS;
        } else if difference <= ((king - 1) % 8) {
            return RevealedCheckSearchType::FromE;
        } else if difference % 9 == 0 {
            if (revealed - 1) % 8 < (king - 1) % 8 {
                return RevealedCheckSearchType::FromSE;
            }
        } else if difference % 7 == 0 {
            if (revealed - 1) % 8 > (king - 1) % 8 {
                return RevealedCheckSearchType::FromSW;
            }
        }
    }
    RevealedCheckSearchType::DoNotSearch
}

pub fn revealed_check_bitmapper(king: u8, search: RevealedCheckSearchType) -> u64 {
    use crate::reset::r#const::B_NOT_NW_EDGE;
    use crate::reset::r#const::B_NOT_NE_EDGE;
    use crate::reset::r#const::B_NOT_SW_EDGE;
    use crate::reset::r#const::B_NOT_SE_EDGE;
    use crate::reset::r#const::B_NOT_N_EDGE;
    use crate::reset::r#const::B_NOT_E_EDGE;
    use crate::reset::r#const::B_NOT_W_EDGE;
    use crate::reset::r#const::B_NOT_S_EDGE;

    let b_king: u64 = 0x0000000000000001 << (king - 1);
    let mut b_temp: u64 = b_king;
    let mut b_map: u64 = 0x0000000000000000;
    match search {
        RevealedCheckSearchType::FromN => {
            // Go N from the king
            while b_temp & B_NOT_N_EDGE != 0 {
                b_temp <<= 8;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNE => {
            // Go NE from the king
            while b_temp & B_NOT_NE_EDGE != 0 {
                b_temp <<= 7;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromE => {
            // Go E from the king
            while b_temp & B_NOT_E_EDGE != 0 {
                b_temp >>= 1;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSE => {
            // Go SE from the king
            while b_temp & B_NOT_SE_EDGE != 0 {
                b_temp >>= 9;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromS => {
            // Go S from the king
            while b_temp & B_NOT_S_EDGE != 0 {
                b_temp >>= 8;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSW => {
            // Go SW from the king
            while b_temp & B_NOT_SW_EDGE != 0 {
                b_temp >>= 7;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromW => {
            // Go W from the king
            while b_temp & B_NOT_W_EDGE != 0 {
                b_temp <<= 1;
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNW => {
            // Go NW from the king
            while b_temp & B_NOT_NW_EDGE != 0 {
                b_temp <<= 9;
                b_map |= b_temp;
            }
        },
        _ => {},
    }
    b_map
}

lazy_static! {
    static ref SEARCH_TYPE_INDEX: HashMap<RevealedCheckSearchType,u8> = {
        let mut map = HashMap::new();
        map.insert(RevealedCheckSearchType::DoNotSearch,0);
        map.insert(RevealedCheckSearchType::FromN,1);
        map.insert(RevealedCheckSearchType::FromNE,2);
        map.insert(RevealedCheckSearchType::FromE,3);
        map.insert(RevealedCheckSearchType::FromSE,4);
        map.insert(RevealedCheckSearchType::FromS,5);
        map.insert(RevealedCheckSearchType::FromSW,6);
        map.insert(RevealedCheckSearchType::FromW,7);
        map.insert(RevealedCheckSearchType::FromNW,8);
        map
    };

    static ref REVEALED_CHECK_ROUTES: Vec<Vec<RevealedCheckSearchType>> = {
        let mut vec: Vec<Vec<RevealedCheckSearchType>> = Vec::new();

        let blank: Vec<RevealedCheckSearchType> = Vec::new();
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

    static ref REVEALED_CHECK_BITMAPS: Vec<Vec<u64>> = {
        let mut vec: Vec<Vec<u64>> = Vec::new();

        let blank: Vec<u64> = Vec::new();
        vec.push(blank); // push a blank at index 0

        for king in 1..65 { // indexes 1 to 64
            let mut bit_strings: Vec<u64> = Vec::new();
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::DoNotSearch));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromN));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromNE));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromE));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromSE));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromS));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromSW));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromW));
            bit_strings.push(revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromNW));
            vec.push(bit_strings);
        }
        vec
    };
}

impl Reset {

    /// Considering the move made in this Reset, return `false` if check was revealed and `true` if
    /// the specified side is safe (black = `0`, white = `1`).
    ///
    /// Someday, king_square won't be needed by this method, but for now it's there for performance
    /// reasons.
    pub fn is_safe_from_revealed_check(&mut self, king_square: u8, from_square: u8, king_color: u8) -> bool {
        use crate::reset::safe_revealed::revealed_check_router;
        use crate::reset::safe_revealed::RevealedCheckSearchType;

        let search_type = &REVEALED_CHECK_ROUTES[king_square as usize][from_square as usize];
        println!("Search Type == {:?}",search_type);
        if matches!(search_type,RevealedCheckSearchType::DoNotSearch) {
            return true;
        }

        let b_opponent: u64 = if king_color == WHITE {
            self.b_black()
        } else {
            self.b_white
        };
        let b_others: u64 = self.b_pawns | self.b_knights | self.b_kings;
        match search_type {
            RevealedCheckSearchType::FromN => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][1];
                if b_opponent & !(b_others | self.b_bishops) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromNE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][2];
                if b_opponent & !(b_others | self.b_rooks) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][3];
                if b_opponent & !(b_others | self.b_bishops) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromSE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][4];
                if b_opponent & !(b_others | self.b_rooks) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromS => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][5];
                if b_opponent & !(b_others | self.b_bishops) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromSW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][6];
                if b_opponent & !(b_others | self.b_rooks) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][7];
                if b_opponent & !(b_others | self.b_bishops) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::FromNW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][8];
                if b_opponent & !(b_others | self.b_rooks) == 0 {
                    return true
                }
            },
            RevealedCheckSearchType::DoNotSearch => {
                // Can't get here
            }
        }

        false
    }

        
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;
    use crate::reset::safe_revealed::revealed_check_router;
    use crate::reset::safe_revealed::RevealedCheckSearchType;
    use crate::reset::r#const::BLACK;
    use crate::reset::r#const::WHITE;

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn search_type_index_1() {
        use crate::reset::safe_revealed::SEARCH_TYPE_INDEX;
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::DoNotSearch],0);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromN],1);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromNE],2);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromE],3);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromSE],4);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromS],5);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromSW],6);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromW],7);
        assert_eq!(SEARCH_TYPE_INDEX[&RevealedCheckSearchType::FromNW],8);
    }

    #[test]
    fn revealed_check_router_1() {
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
    fn revealed_check_router_from_east() {
        assert_eq!(revealed_check_router(40,39),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,38),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,37),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,36),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,35),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(40,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(39,38),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(39,37),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(39,36),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(39,35),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(39,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,37),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,36),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,35),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(38,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(37,36),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(37,35),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(37,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(37,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(36,35),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(36,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(36,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(35,34),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(35,33),RevealedCheckSearchType::FromE);
        assert_eq!(revealed_check_router(34,33),RevealedCheckSearchType::FromE);
    }

    #[test]
    fn revealed_check_router_from_west() {
        assert_eq!(revealed_check_router(32,33),RevealedCheckSearchType::DoNotSearch);
        assert_eq!(revealed_check_router(33,40),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(39,40),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(38,40),RevealedCheckSearchType::FromW);
        assert_eq!(revealed_check_router(32,40),RevealedCheckSearchType::FromN);
    }

    #[test]
    fn revealed_check_routes() {
        // Trusts revealed_check_router, this just ensures that the matrix matches the function
        use crate::reset::safe_revealed::REVEALED_CHECK_ROUTES;
        for king in 1..65 {
            for revealed in 1..65 {
                println!("king == {}, revealed = {}",king,revealed);
                assert_eq!(REVEALED_CHECK_ROUTES[king][revealed],revealed_check_router(king as u8,revealed as u8));
            }
        }
    }

    #[test]
    fn revealed_check_bitmapper_1() {
        use crate::reset::safe_revealed::revealed_check_bitmapper;
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromN),0x0101010101000000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromNE),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromE),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromSE),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromS),0x0000000000000101);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromSW),0x0000000000000204);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromW),0x0000000000fe0000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::FromNW),0x2010080402000000);
        assert_eq!(revealed_check_bitmapper(17,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);

        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromN),0x8080808080000000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromNE),0x0408102040000000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromE),0x00000000007f0000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromSE),0x0000000000004020);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromS),0x0000000000008080);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromSW),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromW),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::FromNW),0x0000000000000000);
        assert_eq!(revealed_check_bitmapper(24,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);

        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromN),0x1010100000000000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromNE),0x0204080000000000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromE),0x0000000f00000000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromSE),0x0000000008040201);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromS),0x0000000010101010);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromSW),0x0000000020408000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromW),0x000000e000000000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::FromNW),0x8040200000000000);
        assert_eq!(revealed_check_bitmapper(37,RevealedCheckSearchType::DoNotSearch),0x0000000000000000);
    }

    #[test]
    fn revealed_check_bitmaps() {
        // Trusts revealed_check_bitmapper, this just ensures that the matrix matches the function
        use crate::reset::safe_revealed::revealed_check_bitmapper;
        use crate::reset::safe_revealed::SEARCH_TYPE_INDEX;
        use crate::reset::safe_revealed::REVEALED_CHECK_BITMAPS;
        for king in 1..65 {
            println!("king == {}",king);
            assert_eq!(REVEALED_CHECK_BITMAPS[king][0],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::DoNotSearch));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][1],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromN));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][2],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromNE));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][3],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromE));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][4],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromSE));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][5],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromS));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][6],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromSW));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][7],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromW));
            assert_eq!(REVEALED_CHECK_BITMAPS[king][8],revealed_check_bitmapper(king as u8,RevealedCheckSearchType::FromNW));
        }
    }

    #[test]
    fn revealed_check_from_n() {
        let mut r = prep_board("2K5/4Q3/8/8/2n5/8/8/4k3 w - - 0 1");
        let from: u8 = utils::convert_square_to_number("e3".to_string());
        let king: u8 = utils::convert_square_to_number("e1".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,BLACK));
    }

    #[test]
    fn revealed_check_from_ne() {
        let mut r = prep_board("5BK1/7P/8/pp6/8/k7/8/8 w - - 0 1");
        let from: u8 = utils::convert_square_to_number("b4".to_string());
        let king: u8 = utils::convert_square_to_number("a3".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,BLACK));
    }

    #[test]
    fn revealed_check_from_e() {
        let mut r = prep_board("8/8/1R6/K1r4k/8/8/8/8 b - - 0 1");
        let from: u8 = utils::convert_square_to_number("b5".to_string());
        let king: u8 = utils::convert_square_to_number("a5".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,WHITE));
    }

    #[test]
    fn revealed_check_from_se() {
        let mut r = prep_board("6Q1/7P/4N3/kp2K3/8/8/7q/8 b - - 0 1");
        let from: u8 = utils::convert_square_to_number("f4".to_string());
        let king: u8 = utils::convert_square_to_number("e5".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,WHITE));
    }

    #[test]
    fn revealed_check_from_s() {
        let mut r = prep_board("2k5/4K3/8/8/2N5/8/8/4r3 w - - 0 1");
        let from: u8 = utils::convert_square_to_number("e5".to_string());
        let king: u8 = utils::convert_square_to_number("e7".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,WHITE));
    }

    #[test]
    fn revealed_check_from_sw() {
        let mut r = prep_board("r2qkb1r/1pp1pppp/n2p1n2/pB6/6b1/5P1P/PPPP4/RNBQK1NR w KQkq - 0 1");
        let from: u8 = utils::convert_square_to_number("d7".to_string());
        let king: u8 = utils::convert_square_to_number("e8".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,BLACK));
    }

    #[test]
    fn revealed_check_from_w() {
        let mut r = prep_board("6K1/8/7N/8/R6k/8/8/8 b - - 0 1");
        let from: u8 = utils::convert_square_to_number("g4".to_string());
        let king: u8 = utils::convert_square_to_number("h4".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,BLACK));
    }

    #[test]
    fn revealed_check_from_nw() {
        let mut r = prep_board("r3kb1r/1pp1pppp/n1p2n2/pB6/1q1P2b1/5P1P/PPP5/RNBQK1NR b KQkq d3 0 1");
        let from: u8 = utils::convert_square_to_number("d2".to_string());
        let king: u8 = utils::convert_square_to_number("e1".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,WHITE));

        let mut r = prep_board("1KB5/P7/8/6pp/8/7k/8/8 w - - 0 1");
        let from: u8 = utils::convert_square_to_number("g4".to_string());
        let king: u8 = utils::convert_square_to_number("h3".to_string());
        assert!(!r.is_safe_from_revealed_check(king,from,BLACK));
    }

}
