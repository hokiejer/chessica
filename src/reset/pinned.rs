use std::process;
use crate::reset::Reset;
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
use crate::reset::safe_revealed::IS_SAFE_FROM_REVEALED_CHECK_FUNCTIONS;

pub const PIN_DIMENSION_UNSET: u8 = 0x00;
pub const PIN_DIMENSION_NONE: u8 =  0x01;
pub const PIN_DIMENSION_NS: u8 =    0x02;
pub const PIN_DIMENSION_EW: u8 =    0x04;
pub const PIN_DIMENSION_NESW: u8 =  0x08;
pub const PIN_DIMENSION_SENW: u8 =  0x10;

pub const PIN_MATCH_NS: u8 =   0xfc;
pub const PIN_MATCH_EW: u8 =   0xfa;
pub const PIN_MATCH_NESW: u8 = 0xf6;
pub const PIN_MATCH_SENW: u8 = 0xee;
pub const PIN_MATCH_NONE: u8 = 0xfe;

static PIN_DIMENSIONS: &'static [u8] = &[
            PIN_DIMENSION_NONE,
            PIN_DIMENSION_NS,
            PIN_DIMENSION_NESW,
            PIN_DIMENSION_EW,
            PIN_DIMENSION_SENW,
            PIN_DIMENSION_NS,
            PIN_DIMENSION_NESW,
            PIN_DIMENSION_EW,
            PIN_DIMENSION_SENW,
];
impl Reset {

    /// Considering the move made in this Reset, return `false` if check was revealed and `true` if
    /// the specified side is safe (black = `0`, white = `1`).
    ///
    /// Someday, king_square won't be needed by this method, but for now it's there for performance
    /// reasons.
    pub fn set_current_piece_pin_dimension(&mut self) {

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
            self.pin_dimension = PIN_DIMENSION_NONE;
            return;
        }

        let b_attacks: u64;
        let mut index: u8;
        let b_others: u64 = self.b_pawns | self.b_knights | self.b_kings;
        match search_type {
            RevealedCheckSearchType::DoNotSearch => {
                return; //Will not get here
            },
            RevealedCheckSearchType::FromN => {
                b_opponents &= !(b_others | self.b_bishops);
                index = 1;
            },
            RevealedCheckSearchType::FromNE => {
                b_opponents &= !(b_others | self.b_rooks);
                index = 2;
            },
            RevealedCheckSearchType::FromE => {
                b_opponents &= !(b_others | self.b_bishops);
                index = 3;
            },
            RevealedCheckSearchType::FromSE => {
                b_opponents &= !(b_others | self.b_rooks);
                index = 4;
            },
            RevealedCheckSearchType::FromS => {
                b_opponents &= !(b_others | self.b_bishops);
                index = 5;
            },
            RevealedCheckSearchType::FromSW => {
                b_opponents &= !(b_others | self.b_rooks);
                index = 6;
            },
            RevealedCheckSearchType::FromW => {
                b_opponents &= !(b_others | self.b_bishops);
                index = 7;
            },
            RevealedCheckSearchType::FromNW => {
                b_opponents &= !(b_others | self.b_rooks);
                index = 8;
            },
        }
        b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][index as usize];
        if b_attacks & b_opponents == 0 {
            self.pin_dimension = PIN_DIMENSION_NONE;
            return;
        }
        let b_board: u64 = self.b_all & !self.b_current_piece;
        if !(IS_SAFE_FROM_REVEALED_CHECK_FUNCTIONS[index as usize])(king_square,b_board,b_opponents) {
            self.pin_dimension = PIN_DIMENSIONS[index as usize];
            return;
        }
        self.pin_dimension = PIN_DIMENSION_NONE;
    }
}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::Reset;
    use crate::utils;
    use crate::reset::safe_revealed::revealed_check_router;
    use crate::reset::safe_revealed::RevealedCheckSearchType;

    use crate::reset::pinned::PIN_DIMENSION_NONE;
    use crate::reset::pinned::PIN_DIMENSION_NS;
    use crate::reset::pinned::PIN_DIMENSION_EW;
    use crate::reset::pinned::PIN_DIMENSION_NESW;
    use crate::reset::pinned::PIN_DIMENSION_SENW;

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
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NS);

        let mut r = prep_board("8/k3r3/4n3/4B3/4K3/8/8/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("e5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("e5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_ne() {
        let mut r = prep_board("5BK1/7P/8/p7/1p6/k7/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b4".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NESW);

        let mut r = prep_board("6KQ/6Q1/5B2/4Pr2/3p4/2k5/8/5q2 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d4".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_e() {
        let mut r = prep_board("8/8/1R6/K1r4k/8/8/8/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_EW);

        let mut r = prep_board("8/8/8/KRr4k/8/8/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_EW);
    }

    #[test]
    fn pinned_check_from_se() {
        let mut r = prep_board("6Q1/7P/8/kp2K3/5N2/8/7q/8 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("f4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("f4".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_SENW);

        let mut r = prep_board("1k6/2q5/5p2/4r3/5Q2/6K1/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c7".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_s() {
        let mut r = prep_board("2k5/4K3/8/4N3/8/8/8/4r3 w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("e5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("e5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NS);

        let mut r = prep_board("1k6/1r6/8/8/8/2pK4/8/1Q6 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b7".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NS);

        let mut r = prep_board("1k6/8/8/8/8/2pK4/1r6/1BQ5 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b2".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_sw() {
        let mut r = prep_board("r2qkb1r/1ppppppp/n4n2/pB6/6b1/5P1P/PPPP4/RNBQK1NR b KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d7".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NESW);

        let mut r = prep_board("8/6k1/5p2/4r3/2KB4/8/8/Q7 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("f6".to_string());
        r.bi_current_piece = utils::convert_square_to_number("f6".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_w() {
        let mut r = prep_board("6K1/8/8/8/R5nk/8/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("g4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("g4".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_EW);

        let mut r = prep_board("8/6k1/5p2/r1RK4/8/8/8/7Q w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_EW);

        let mut r = prep_board("8/6k1/5p2/b1RK4/8/8/8/7Q w - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("c5".to_string());
        r.bi_current_piece = utils::convert_square_to_number("c5".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);
    }

    #[test]
    fn pinned_check_from_nw() {
        let mut r = prep_board("r3kb1r/1pp1pppp/n1p2n2/pB6/1q4b1/5P1P/PPPPN3/RNBQK2R w KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d2".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_SENW);

        let mut r = prep_board("1KB5/P7/8/7p/6p1/7k/8/8 b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("g4".to_string());
        r.bi_current_piece = utils::convert_square_to_number("g4".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_SENW);

        let mut r = prep_board("r3kb1r/1pp1pppp/n1p2n2/pB6/1q4b1/2P2P1P/PP1P4/RNBQK1NR w KQkq - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("d2".to_string());
        r.bi_current_piece = utils::convert_square_to_number("d2".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_NONE);

        let mut r = prep_board("QK6/1b5p/8/8/8/8/8/7k b - - 0 1");
        r.b_current_piece = utils::convert_square_to_bitstring("b7".to_string());
        r.bi_current_piece = utils::convert_square_to_number("b7".to_string());
        r.set_current_piece_pin_dimension();
        assert_eq!(r.pin_dimension,PIN_DIMENSION_SENW);
    }

}

