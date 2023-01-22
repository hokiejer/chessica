use crate::reset::Reset;

use crate::reset::r#const::WHITE;

use crate::reset::r#const::B_NOT_N_EDGE;
use crate::reset::r#const::B_NOT_NE_EDGE;
use crate::reset::r#const::B_NOT_E_EDGE;
use crate::reset::r#const::B_NOT_SE_EDGE;
use crate::reset::r#const::B_NOT_S_EDGE;
use crate::reset::r#const::B_NOT_SW_EDGE;
use crate::reset::r#const::B_NOT_W_EDGE;
use crate::reset::r#const::B_NOT_NW_EDGE;

//
//Consider the attacking piece
//
//If it's a king, there will be a kingstars bitmap to mask it against
//  If it matches, the king is in check
//If it's a knight, there will be a knightstars bitmap to mask it against
//  If it matches, the king is in check
//If it's a white pawn, there will be a whitepawnstars bitmap to mask it against
//  If it matches, the king is in check
//If it's a black pawn, there will be a blackpawnstars bitmap to mask it against
//  If it matches, the king is in check
//If it's a bishop, there are two bitmaps: one to see if the bishop is on a line,
//and one (if so) to look for empty squares.  
//  If those squares are empty, then the king is in check.
//
#[derive(PartialEq,Eq,Hash,Debug)]
pub enum DirectCheckSearchType {
    DoNotSearch,
    LocalStraight,
    LocalDiagonal,
    Knight,
    LongDistanceFromN,
    LongDistanceFromNE,
    LongDistanceFromE,
    LongDistanceFromSE,
    LongDistanceFromS,
    LongDistanceFromSW,
    LongDistanceFromW,
    LongDistanceFromNW,
}

pub fn direct_check_router(king: u8, attacker: u8) -> DirectCheckSearchType {
    if king == attacker {
        return DirectCheckSearchType::DoNotSearch;
    }

    if king < attacker { // Attack from W, NW, N, or NE
        let difference: u8 = attacker - king;

        if difference == 15 && ((king - 1) % 8) > 0 {
            return DirectCheckSearchType::Knight; //Knight 0100
        }
        if difference == 6 && ((king - 1) % 8) > 1 {
            return DirectCheckSearchType::Knight; //Knight 0200
        }
        if difference == 10 && ((king - 1) % 8) < 6 {
            return DirectCheckSearchType::Knight; //Knight 1000
        }
        if difference == 17 && ((king - 1) % 8) < 7 {
            return DirectCheckSearchType::Knight; //Knight 1100
        }

        if difference == 9 && ((king - 1) % 8) < 7 {
            return DirectCheckSearchType::LocalDiagonal; //Local NW
        }
        if difference == 8 {
            return DirectCheckSearchType::LocalStraight; //Local N
        }
        if difference == 7 && ((king - 1) % 8) > 0 {
            return DirectCheckSearchType::LocalDiagonal; //Local NE
        }
        if difference == 1 && ((king - 1) % 8) < 7 {
            return DirectCheckSearchType::LocalStraight; //Local W
        }
        if difference % 8 == 0 {
            return DirectCheckSearchType::LongDistanceFromN;
        } else if difference <= 8 - ((king - 1) % 8) - 1 {
            return DirectCheckSearchType::LongDistanceFromW;
        } else if difference % 9 == 0 {
            if (attacker - 1) % 8 > (king - 1) % 8 {
                return DirectCheckSearchType::LongDistanceFromNW;
            }
        } else if difference % 7 == 0 {
            if (attacker - 1) % 8 < (king - 1) % 8 {
                return DirectCheckSearchType::LongDistanceFromNE;
            }
        }
    } else { // Attack from E, SE, S, or SW
        let difference: u8 = king - attacker;

        if difference == 15 && ((king - 1) % 8) < 7 {
            return DirectCheckSearchType::Knight; //Knight 0700
        }
        if difference == 6 && ((king - 1) % 8) < 6 {
            return DirectCheckSearchType::Knight; //Knight 0800
        }
        if difference == 10 && ((king - 1) % 8) > 1 {
            return DirectCheckSearchType::Knight; //Knight 0400
        }
        if difference == 17 && ((king - 1) % 8) > 0 {
            return DirectCheckSearchType::Knight; //Knight 0500
        }
        if difference == 1 && ((king - 1) % 8) > 0 {
            return DirectCheckSearchType::LocalStraight; //Local E
        }
        if difference == 7 && ((king - 1) % 8) < 7 {
            return DirectCheckSearchType::LocalDiagonal; //Local SW
        }
        if difference == 8 {
            return DirectCheckSearchType::LocalStraight; //Local S
        }
        if difference == 9 && ((king - 1) % 8) > 0 {
            return DirectCheckSearchType::LocalDiagonal; //Local SE
        }
        if difference % 8 == 0 {
            return DirectCheckSearchType::LongDistanceFromS;
        } else if difference <= ((king - 1) % 8) {
            return DirectCheckSearchType::LongDistanceFromE;
        } else if difference % 9 == 0 {
            if (attacker - 1) % 8 < (king - 1) % 8 {
                return DirectCheckSearchType::LongDistanceFromSE;
            }
        } else if difference % 7 == 0 {
            if (attacker - 1) % 8 > (king - 1) % 8 {
                return DirectCheckSearchType::LongDistanceFromSW;
            }
        }
    }
    DirectCheckSearchType::DoNotSearch
}

pub fn long_distance_check_bitmapper(king: u8, attacker: u8) -> u64 {
    use crate::reset::safe_revealed::revealed_check_router;
    use crate::reset::safe_revealed::RevealedCheckSearchType;
    let b_king: u64 = 0x0000000000000001 << (king - 1);
    let b_attacker: u64 = 0x0000000000000001 << (attacker - 1);
    let mut b_temp: u64 = b_king;
    let mut b_map: u64 = 0x0000000000000000;
    match revealed_check_router(king, attacker) {
        RevealedCheckSearchType::FromN => {
            // Go N from the king
            while b_temp & B_NOT_N_EDGE != 0 {
                b_temp <<= 8;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNE => {
            // Go NE from the king
            while b_temp & B_NOT_NE_EDGE != 0 {
                b_temp <<= 7;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromE => {
            // Go E from the king
            while b_temp & B_NOT_E_EDGE != 0 {
                b_temp >>= 1;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSE => {
            // Go SE from the king
            while b_temp & B_NOT_SE_EDGE != 0 {
                b_temp >>= 9;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromS => {
            // Go S from the king
            while b_temp & B_NOT_S_EDGE != 0 {
                b_temp >>= 8;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromSW => {
            // Go SW from the king
            while b_temp & B_NOT_SW_EDGE != 0 {
                b_temp >>= 7;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromW => {
            // Go W from the king
            while b_temp & B_NOT_W_EDGE != 0 {
                b_temp <<= 1;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        RevealedCheckSearchType::FromNW => {
            // Go NW from the king
            while b_temp & B_NOT_NW_EDGE != 0 {
                b_temp <<= 9;
                if b_temp == b_attacker {
                    break;
                }
                b_map |= b_temp;
            }
        },
        _ => {},
    }
    b_map
}

lazy_static! {

    static ref DIRECT_CHECK_ROUTES: Vec<Vec<DirectCheckSearchType>> = {
        let mut vec: Vec<Vec<DirectCheckSearchType>> = Vec::new();

        let blank: Vec<DirectCheckSearchType> = Vec::new();
        vec.push(blank); // push a blank at index 0

        for king in 1..65 { // indexes 1 to 64
            let mut mini_router: Vec<DirectCheckSearchType> = Vec::new();
            mini_router.push(DirectCheckSearchType::DoNotSearch); // push a blank at index 0
            for attacker in 1..65 { // indexes 1 to 64
                mini_router.push(direct_check_router(king as u8,attacker as u8));
            }
            vec.push(mini_router);
        }
        vec
    };

    static ref LONG_DISTANCE_CHECK_BITMAPS: Vec<Vec<u64>> = {
        let mut vec: Vec<Vec<u64>> = Vec::new();

        let blank: Vec<u64> = Vec::new();
        vec.push(blank); // push a blank at index 0

        for king in 1..65 { // indexes 1 to 64
            let mut bit_strings: Vec<u64> = Vec::new();
            bit_strings.push(0); // push a blank at index 0
            for attacker in 1..65 { // indexes 1 to 64
                bit_strings.push(long_distance_check_bitmapper(king as u8, attacker as u8));
            }
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
    pub fn is_safe_from_direct_check(&mut self, king_square: u8, attack_square: u8, king_color: u8) -> bool {

        let search_type = &DIRECT_CHECK_ROUTES[king_square as usize][attack_square as usize];
        if matches!(search_type,DirectCheckSearchType::DoNotSearch) {
            return true;
        }

        let mut b_opponents: u64 = if king_color == WHITE {
            self.b_black()
        } else {
            self.b_white
        };
        let b_others: u64 = self.b_pawns | self.b_knights | self.b_kings;
        match search_type {
            DirectCheckSearchType::Knight => {
                if self.b_to & b_opponents & self.b_knights != 0 {
                    return false;
                }
            },
            DirectCheckSearchType::LocalStraight => {
                let b_attackers: u64 = self.b_all & !(self.b_pawns & self.b_knights & self.b_bishops);
                if self.b_to & b_opponents & b_attackers != 0 {
                    return false;
                }
            },
            DirectCheckSearchType::LocalDiagonal => {
                if king_color == WHITE {
                    if attack_square > king_square {
                        let b_attackers: u64 = self.b_all & !(self.b_knights & self.b_rooks);
                        if self.b_to & b_opponents & b_attackers != 0 {
                            return false;
                        }
                    } else {
                        let b_attackers: u64 = self.b_all & !(self.b_pawns & self.b_knights & self.b_rooks);
                        if self.b_to & b_opponents & b_attackers != 0 {
                            return false;
                        }
                    }
                } else {
                    if attack_square < king_square {
                        let b_attackers: u64 = self.b_all & !(self.b_knights & self.b_rooks);
                        if self.b_to & b_opponents & b_attackers != 0 {
                            return false;
                        }
                    } else {
                        let b_attackers: u64 = self.b_all & !(self.b_pawns & self.b_knights & self.b_rooks);
                        if self.b_to & b_opponents & b_attackers != 0 {
                            return false;
                        }
                    }
                }
            },
            DirectCheckSearchType::LongDistanceFromN | 
            DirectCheckSearchType::LongDistanceFromE |
            DirectCheckSearchType::LongDistanceFromS |
            DirectCheckSearchType::LongDistanceFromW => {
                b_opponents &= !(b_others | self.b_bishops);
                if self.b_to & b_opponents == 0 {
                    return true;
                }
                let b_empty_squares = LONG_DISTANCE_CHECK_BITMAPS[king_square as usize][attack_square as usize];
                if self.b_all & b_empty_squares == 0 {
                    return false;
                }
            },
            DirectCheckSearchType::LongDistanceFromNE |
            DirectCheckSearchType::LongDistanceFromSE |
            DirectCheckSearchType::LongDistanceFromSW |
            DirectCheckSearchType::LongDistanceFromNW => {
                b_opponents &= !(b_others | self.b_rooks);
                if self.b_to & b_opponents == 0 {
                    return true;
                }
                let b_empty_squares = LONG_DISTANCE_CHECK_BITMAPS[king_square as usize][attack_square as usize];
                if self.b_all & b_empty_squares == 0 {
                    return false;
                }
            },
            DirectCheckSearchType::DoNotSearch => {
                // Can't get here
            }
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;
    use crate::reset::safe_direct::direct_check_router;
    use crate::reset::safe_direct::DirectCheckSearchType;
    use crate::reset::r#const::BLACK;
    use crate::reset::r#const::WHITE;

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn direct_check_router_1() {
        assert_eq!(direct_check_router(20, 1),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20, 2),DirectCheckSearchType::LongDistanceFromSE);
        assert_eq!(direct_check_router(20, 3),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20, 4),DirectCheckSearchType::LongDistanceFromS);
        assert_eq!(direct_check_router(20, 5),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20, 6),DirectCheckSearchType::LongDistanceFromSW);
        assert_eq!(direct_check_router(20, 7),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20, 8),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20, 9),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,10),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,11),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(20,12),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(20,13),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(20,14),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,15),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,16),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,17),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(20,18),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(20,19),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(20,20),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,21),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(20,22),DirectCheckSearchType::LongDistanceFromW);
        assert_eq!(direct_check_router(20,23),DirectCheckSearchType::LongDistanceFromW);
        assert_eq!(direct_check_router(20,24),DirectCheckSearchType::LongDistanceFromW);
        assert_eq!(direct_check_router(20,25),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,26),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,27),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(20,28),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(20,29),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(20,30),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,31),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,32),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,33),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,34),DirectCheckSearchType::LongDistanceFromNE);
        assert_eq!(direct_check_router(20,35),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,36),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(20,37),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(20,38),DirectCheckSearchType::LongDistanceFromNW);
        assert_eq!(direct_check_router(20,39),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,40),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,41),DirectCheckSearchType::LongDistanceFromNE);
        assert_eq!(direct_check_router(20,42),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,43),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,44),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(20,45),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,46),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,47),DirectCheckSearchType::LongDistanceFromNW);
        assert_eq!(direct_check_router(20,48),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,49),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,50),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,51),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,52),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(20,53),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,54),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,55),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,56),DirectCheckSearchType::LongDistanceFromNW);
        assert_eq!(direct_check_router(20,57),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,58),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,59),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,60),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(20,61),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,62),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,63),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(20,64),DirectCheckSearchType::DoNotSearch);
    }

    #[test]
    fn direct_check_router_2() {
        assert_eq!(direct_check_router(31, 1),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 2),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 3),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 4),DirectCheckSearchType::LongDistanceFromSE);
        assert_eq!(direct_check_router(31, 5),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 6),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 7),DirectCheckSearchType::LongDistanceFromS);
        assert_eq!(direct_check_router(31, 8),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31, 9),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,10),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,11),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,12),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,13),DirectCheckSearchType::LongDistanceFromSE);
        assert_eq!(direct_check_router(31,14),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,15),DirectCheckSearchType::LongDistanceFromS);
        assert_eq!(direct_check_router(31,16),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,17),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,18),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,19),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,20),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,21),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,22),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(31,23),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(31,24),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(31,25),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(31,26),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(31,27),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(31,28),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(31,29),DirectCheckSearchType::LongDistanceFromE);
        assert_eq!(direct_check_router(31,30),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(31,31),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,32),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(31,33),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,34),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,35),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,36),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,37),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,38),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(31,39),DirectCheckSearchType::LocalStraight);
        assert_eq!(direct_check_router(31,40),DirectCheckSearchType::LocalDiagonal);
        assert_eq!(direct_check_router(31,41),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,42),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,43),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,44),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,45),DirectCheckSearchType::LongDistanceFromNE);
        assert_eq!(direct_check_router(31,46),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,47),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(31,48),DirectCheckSearchType::Knight);
        assert_eq!(direct_check_router(31,49),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,50),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,51),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,52),DirectCheckSearchType::LongDistanceFromNE);
        assert_eq!(direct_check_router(31,53),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,54),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,55),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(31,56),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,57),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,58),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,59),DirectCheckSearchType::LongDistanceFromNE);
        assert_eq!(direct_check_router(31,60),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,61),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,62),DirectCheckSearchType::DoNotSearch);
        assert_eq!(direct_check_router(31,63),DirectCheckSearchType::LongDistanceFromN);
        assert_eq!(direct_check_router(31,64),DirectCheckSearchType::DoNotSearch);
    }

    #[test]
    fn direct_check_routes() {
        // Trusts direct_check_router, this just ensures that the matrix matches the function
        use crate::reset::safe_direct::DIRECT_CHECK_ROUTES;
        for king in 1..65 {
            for attacker in 1..65 {
                assert_eq!(DIRECT_CHECK_ROUTES[king][attacker],direct_check_router(king as u8,attacker as u8));
            }
        }
    }

    #[test]
    fn long_distance_check_bitmaps_1() {
        use crate::reset::safe_direct::long_distance_check_bitmapper;
        assert_eq!(long_distance_check_bitmapper(17, 1),0x0000000000000100);
        assert_eq!(long_distance_check_bitmapper(17, 2),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 3),0x0000000000000200);
        assert_eq!(long_distance_check_bitmapper(17, 4),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 5),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 6),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 7),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 8),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17, 9),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,10),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,11),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,12),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,13),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,14),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,15),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,16),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,17),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,18),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,19),0x0000000000020000);
        assert_eq!(long_distance_check_bitmapper(17,20),0x0000000000060000);
        assert_eq!(long_distance_check_bitmapper(17,21),0x00000000000e0000);
        assert_eq!(long_distance_check_bitmapper(17,22),0x00000000001e0000);
        assert_eq!(long_distance_check_bitmapper(17,23),0x00000000003e0000);
        assert_eq!(long_distance_check_bitmapper(17,24),0x00000000007e0000);
        assert_eq!(long_distance_check_bitmapper(17,25),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,26),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,27),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,28),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,29),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,30),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,31),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,32),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,33),0x0000000001000000);
        assert_eq!(long_distance_check_bitmapper(17,34),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,35),0x0000000002000000);
        assert_eq!(long_distance_check_bitmapper(17,36),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,37),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,38),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,39),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,40),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,41),0x0000000101000000);
        assert_eq!(long_distance_check_bitmapper(17,42),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,43),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,44),0x0000000402000000);
        assert_eq!(long_distance_check_bitmapper(17,45),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,46),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,47),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,48),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,49),0x0000010101000000);
        assert_eq!(long_distance_check_bitmapper(17,50),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,51),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,52),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,53),0x0000080402000000);
        assert_eq!(long_distance_check_bitmapper(17,54),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,55),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,56),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,57),0x0001010101000000);
        assert_eq!(long_distance_check_bitmapper(17,58),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,59),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,60),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,61),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,62),0x0010080402000000);
        assert_eq!(long_distance_check_bitmapper(17,63),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(17,64),0x0000000000000000);
    }

    #[test]
    fn long_distance_check_bitmaps_2() {
        use crate::reset::safe_direct::long_distance_check_bitmapper;
        assert_eq!(long_distance_check_bitmapper(46, 1),0x0000001008040200);
        assert_eq!(long_distance_check_bitmapper(46, 2),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 3),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 4),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 5),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 6),0x0000002020202000);
        assert_eq!(long_distance_check_bitmapper(46, 7),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 8),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46, 9),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,10),0x0000001008040000);
        assert_eq!(long_distance_check_bitmapper(46,11),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,12),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,13),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,14),0x0000002020200000);
        assert_eq!(long_distance_check_bitmapper(46,15),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,16),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,17),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,18),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,19),0x0000001008000000);
        assert_eq!(long_distance_check_bitmapper(46,20),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,21),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,22),0x0000002020000000);
        assert_eq!(long_distance_check_bitmapper(46,23),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,24),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,25),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,26),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,27),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,28),0x0000001000000000);
        assert_eq!(long_distance_check_bitmapper(46,29),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,30),0x0000002000000000);
        assert_eq!(long_distance_check_bitmapper(46,31),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,32),0x0000004000000000);
        assert_eq!(long_distance_check_bitmapper(46,33),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,34),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,35),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,36),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,37),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,38),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,39),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,40),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,41),0x00001e0000000000);
        assert_eq!(long_distance_check_bitmapper(46,42),0x00001c0000000000);
        assert_eq!(long_distance_check_bitmapper(46,43),0x0000180000000000);
        assert_eq!(long_distance_check_bitmapper(46,44),0x0000100000000000);
        assert_eq!(long_distance_check_bitmapper(46,45),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,46),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,47),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,48),0x0000400000000000);
        assert_eq!(long_distance_check_bitmapper(46,49),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,50),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,51),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,52),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,53),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,54),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,55),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,56),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,57),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,58),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,59),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,60),0x0010000000000000);
        assert_eq!(long_distance_check_bitmapper(46,61),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,62),0x0020000000000000);
        assert_eq!(long_distance_check_bitmapper(46,63),0x0000000000000000);
        assert_eq!(long_distance_check_bitmapper(46,64),0x0040000000000000);
    }

    #[test]
    fn long_distance_check_bitmaps() {
        // Trusts direct_check_bitmapper, this just ensures that the matrix matches the function
        use crate::reset::safe_direct::long_distance_check_bitmapper;
        use crate::reset::safe_direct::LONG_DISTANCE_CHECK_BITMAPS;
        for king in 1..65 {
            for attacker in 1..65 {
                let bitmap = long_distance_check_bitmapper(king as u8,attacker as u8);
                assert_eq!(LONG_DISTANCE_CHECK_BITMAPS[king][attacker],bitmap);
            }
        }
    }

    #[test]
    fn direct_check_from_n() {
        let mut r = prep_board("2K5/4Q3/8/8/2n5/8/8/4k3 w - - 0 1");
        let king: u8 = utils::convert_square_to_number("e1".to_string());
        let attacker: u8 = utils::convert_square_to_number("e7".to_string());
        r.b_to = utils::convert_square_to_bitstring("e7".to_string());
        assert!(!r.is_safe_from_direct_check(king,attacker,BLACK));

        let mut r = prep_board("2K5/2R5/8/8/8/8/2q5/4k3 w - - 0 1");
        let king: u8 = utils::convert_square_to_number("c8".to_string());
        let attacker: u8 = utils::convert_square_to_number("c2".to_string());
        r.b_to = utils::convert_square_to_bitstring("c2".to_string());
        assert!(r.is_safe_from_direct_check(king,attacker,WHITE));
    }

    #[test]
    fn direct_check_from_ne() {
        let mut r = prep_board("2K5/8/8/8/7B/3q4/3N4/4k3 b - - 0 1");
        let king: u8 = utils::convert_square_to_number("e1".to_string());
        let attacker: u8 = utils::convert_square_to_number("h4".to_string());
        r.b_to = utils::convert_square_to_bitstring("h4".to_string());
        assert!(!r.is_safe_from_direct_check(king,attacker,BLACK));

        let mut r = prep_board("7b/6q1/8/8/6B1/8/3N4/K3k3 w - - 0 1");
        let king: u8 = utils::convert_square_to_number("a1".to_string());
        let attacker: u8 = utils::convert_square_to_number("g7".to_string());
        r.b_to = utils::convert_square_to_bitstring("g7".to_string());
        assert!(!r.is_safe_from_direct_check(king,attacker,WHITE));
    }

}
