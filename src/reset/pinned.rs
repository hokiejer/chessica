use std::process;
use crate::reset::Reset;
use crate::reset::r#const::WHITE;
use crate::reset::safe_revealed::RevealedCheckSearchType;
use crate::reset::safe_revealed::REVEALED_CHECK_BITMAPS;
use crate::reset::safe_revealed::REVEALED_CHECK_ROUTES;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_n;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_ne;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_e;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_se;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_s;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_sw;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_w;
use crate::reset::safe_revealed::is_safe_from_revealed_check_from_nw;

#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
pub enum PinDimension {
    Unset,
    None,
    NS,
    NESW,
    EW,
    SENW,
}

impl Reset {

    /// Considering the move made in this Reset, return `false` if check was revealed and `true` if
    /// the specified side is safe (black = `0`, white = `1`).
    ///
    /// Someday, king_square won't be needed by this method, but for now it's there for performance
    /// reasons.
    pub fn current_piece_pin_dimension(&mut self) -> PinDimension {

        let mut b_opponents: u64 = if self.white_to_move() {
            self.b_black()
        } else {
            self.b_white
        };
        let king_square: u8 = if self.white_to_move() {
            self.white_king_square
        } else {
            self.black_king_square
        };
        let from_square = self.bi_current_piece as usize;

        let search_type = &REVEALED_CHECK_ROUTES[king_square as usize][from_square as usize];
        if matches!(search_type,RevealedCheckSearchType::DoNotSearch) {
            return PinDimension::None;
        }

        let b_others: u64 = self.b_pawns | self.b_knights | self.b_kings;
        let b_board: u64 = self.b_all & !self.b_current_piece;
        match search_type {
            RevealedCheckSearchType::FromN => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][1];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_n(king_square,b_board,b_opponents) {
                    return PinDimension::NS;
                }
            },
            RevealedCheckSearchType::FromNE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][2];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_ne(king_square,b_board,b_opponents) {
                    return PinDimension::NESW;
                }
            },
            RevealedCheckSearchType::FromE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][3];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_e(king_square,b_board,b_opponents) {
                    return PinDimension::EW;
                }
            },
            RevealedCheckSearchType::FromSE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][4];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_se(king_square,b_board,b_opponents) {
                    return PinDimension::SENW;
                }
            },
            RevealedCheckSearchType::FromS => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][5];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_s(king_square,b_board,b_opponents) {
                    return PinDimension::NS;
                }
            },
            RevealedCheckSearchType::FromSW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][6];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_sw(king_square,b_board,b_opponents) {
                    return PinDimension::NESW;
                }
            },
            RevealedCheckSearchType::FromW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][7];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_w(king_square,b_board,b_opponents) {
                    return PinDimension::EW;
                }
            },
            RevealedCheckSearchType::FromNW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][8];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return PinDimension::None;
                }
                if !is_safe_from_revealed_check_from_nw(king_square,b_board,b_opponents) {
                    return PinDimension::SENW;
                }
            },
            RevealedCheckSearchType::DoNotSearch => {
                // Will not be reached
            }
        }
        PinDimension::None
    }

        
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;
    use crate::reset::safe_revealed::revealed_check_router;
    use crate::reset::safe_revealed::RevealedCheckSearchType;
    use crate::reset::pinned::PinDimension;
    use crate::reset::r#const::BLACK;
    use crate::reset::r#const::WHITE;

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn pinned_check_from_n() {
        let mut r = prep_board("2K5/4Q3/8/8/8/4n3/8/4k3 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("e3".to_string());
        r.bi_current_piece = utils::convert_square_to_number("e3".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::NS);

        let mut r = prep_board("8/k3r3/4n3/4B3/4K3/8/8/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("e5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("e5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_ne() {
        let mut r = prep_board("5BK1/7P/8/p7/1p6/k7/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b4".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::NESW);

        let mut r = prep_board("6KQ/6Q1/5B2/4Pr2/3p4/2k5/8/5q2 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d4".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_e() {
        let mut r = prep_board("8/8/1R6/K1r4k/8/8/8/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::EW);

        let mut r = prep_board("8/8/8/KRr4k/8/8/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::EW);
    }

    #[test]
    fn pinned_check_from_se() {
        let mut r = prep_board("6Q1/7P/8/kp2K3/5N2/8/7q/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("f4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("f4".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::SENW);

        let mut r = prep_board("1k6/2q5/5p2/4r3/5Q2/6K1/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c7".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_s() {
        let mut r = prep_board("2k5/4K3/8/4N3/8/8/8/4r3 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("e5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("e5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::NS);

        let mut r = prep_board("1k6/1r6/8/8/8/2pK4/8/1Q6 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b7".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::NS);

        let mut r = prep_board("1k6/8/8/8/8/2pK4/1r6/1BQ5 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b2".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_sw() {
        let mut r = prep_board("r2qkb1r/1ppppppp/n4n2/pB6/6b1/5P1P/PPPP4/RNBQK1NR b KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d7".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::NESW);

        let mut r = prep_board("8/6k1/5p2/4r3/2KB4/8/8/Q7 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("f6".to_string());
        r.bi_current_piece = utils::convert_square_to_number("f6".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_w() {
        let mut r = prep_board("6K1/8/8/8/R5nk/8/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("g4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("g4".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::EW);

        let mut r = prep_board("8/6k1/5p2/r1RK4/8/8/8/7Q w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::EW);

        let mut r = prep_board("8/6k1/5p2/b1RK4/8/8/8/7Q w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);
    }

    #[test]
    fn pinned_check_from_nw() {
        let mut r = prep_board("r3kb1r/1pp1pppp/n1p2n2/pB6/1q4b1/5P1P/PPPPN3/RNBQK2R w KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d2".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::SENW);

        let mut r = prep_board("1KB5/P7/8/7p/6p1/7k/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("g4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("g4".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::SENW);

        let mut r = prep_board("r3kb1r/1pp1pppp/n1p2n2/pB6/1q4b1/2P2P1P/PP1P4/RNBQK1NR w KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d2".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::None);

        let mut r = prep_board("QK6/1b5p/8/8/8/8/8/7k b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b7".to_string());
        assert_eq!(r.current_piece_pin_dimension(),PinDimension::SENW);
    }

}

