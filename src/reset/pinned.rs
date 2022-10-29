use std::process;
use crate::reset::Reset;
use crate::reset::r#const::WHITE;

impl Reset {

    /// Considering the move made in this Reset, return `false` if check was revealed and `true` if
    /// the specified side is safe (black = `0`, white = `1`).
    ///
    /// Someday, king_square won't be needed by this method, but for now it's there for performance
    /// reasons.
    pub fn is_pinned_to_king(&mut self, king_square: u8, from_square: u8, king_color: u8) -> bool {
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

        let search_type = &REVEALED_CHECK_ROUTES[king_square as usize][from_square as usize];
        if matches!(search_type,RevealedCheckSearchType::DoNotSearch) {
            return true;
        }

        let mut b_opponents: u64 = if king_color == WHITE {
            self.b_black()
        } else {
            self.b_white
        };
        let b_others: u64 = self.b_pawns | self.b_knights | self.b_kings;
        let b_board: u64 = self.b_all & !self.b_current_piece;
        match search_type {
            RevealedCheckSearchType::FromN => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][1];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_n(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromNE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][2];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_ne(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][3];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_e(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromSE => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][4];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_se(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromS => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][5];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_s(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromSW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][6];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_sw(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][7];
                b_opponents &= !(b_others | self.b_bishops);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_w(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::FromNW => {
                let b_attacks = REVEALED_CHECK_BITMAPS[king_square as usize][8];
                b_opponents &= !(b_others | self.b_rooks);
                if b_attacks & b_opponents == 0 {
                    return true;
                }
                return is_safe_from_revealed_check_from_nw(king_square,b_board,b_opponents)
            },
            RevealedCheckSearchType::DoNotSearch => {
                // Can't get here
            }
        }
        #[cfg(debug_assertions)]
        {
            println!("Did not expect to get here in revealed check router?!?!");
            println!("Self:");
            self.print();
            process::abort();
        }
        false // Shouldn't get here, but just in case
    }

        
}
