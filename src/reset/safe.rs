use crate::reset::Reset;

impl Reset {

    pub fn white_is_safe(&mut self, b_squares: u64) -> bool {
        self.is_safe(b_squares, 1)
    }
        
    pub fn black_is_safe(&mut self, b_squares: u64) -> bool {
        self.is_safe(b_squares, 0)
    }
        
    /// Determine whether whether a set of squares in a Reset is safe from an opponent's attack
    ///
    /// The `opponent` parameter will be '0' for white and '1' for black.  Returns `true` if the
    /// b_squares are safe and `false` otherwise.
    ///
    /// WARNING: This method does not work for the safety of pawns in an En Passant situation
    ///
    pub fn is_safe(&mut self, b_squares: u64, opponent: u8) -> bool {
        use crate::reset::r#const::B_NOT_NW_EDGE;
        use crate::reset::r#const::B_NOT_NE_EDGE;
        use crate::reset::r#const::B_NOT_SW_EDGE;
        use crate::reset::r#const::B_NOT_SE_EDGE;
        use crate::reset::r#const::B_NOT_N_EDGE;
        use crate::reset::r#const::B_NOT_E_EDGE;
        use crate::reset::r#const::B_NOT_W_EDGE;
        use crate::reset::r#const::B_NOT_S_EDGE;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0100;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0200;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0400;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0500;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0700;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_0800;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_1000;
        use crate::reset::r#const::B_KNIGHT_CAN_MOVE_1100;

        // Pawns
        let b_opponent: u64 = if opponent == 0 { // Black is safe
            // Pawns - Southwest
            if ((b_squares & B_NOT_SW_EDGE) >> 7) & (self.b_pawns & self.b_white) != 0 {
                return false;
            }
            // Pawns - Southeast
            if ((b_squares & B_NOT_SE_EDGE) >> 9) & (self.b_pawns & self.b_white) != 0 {
                return false;
            }
            self.b_white
        } else { // White is safe
            // Pawns - Northwest
            if ((b_squares & B_NOT_NW_EDGE) << 9) & (self.b_pawns & self.b_black()) != 0 {
                return false;
            }
            // Pawns - Northeast
            if ((b_squares & B_NOT_NE_EDGE) << 7) & (self.b_pawns & self.b_black()) != 0 {
                return false;
            }
            self.b_black()
        };

        // Bishop or Queen
        let b_other_stuff: u64 = self.b_pawns | self.b_knights | self.b_kings;
        let b_attackers: u64 = b_opponent & !(b_other_stuff | self.b_rooks);

        if b_attackers != 0 {
            // Bishop or Queen: Northwest
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_NW_EDGE != 0 {
                b_temp = (b_temp & B_NOT_NW_EDGE) << 9;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Northeast
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_NE_EDGE != 0 {
                b_temp = (b_temp & B_NOT_NE_EDGE) << 7;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Southwest
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_SW_EDGE != 0 {
                b_temp = (b_temp & B_NOT_SW_EDGE) >> 7;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Southeast
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_SE_EDGE != 0 {
                b_temp = (b_temp & B_NOT_SE_EDGE) >> 9;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }
        }

        // Rook or Queen
        let b_attackers: u64 = b_opponent & !(b_other_stuff | self.b_bishops);

        if b_attackers != 0 {
            // Rook or Queen: North
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_N_EDGE != 0 {
                b_temp = (b_temp & B_NOT_N_EDGE) << 8;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Rook or Queen: East
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_E_EDGE != 0 {
                b_temp = (b_temp & B_NOT_E_EDGE) >> 1;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Rook or Queen: South
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_S_EDGE != 0 {
                b_temp = (b_temp & B_NOT_S_EDGE) >> 8;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Rook or Queen: West
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_W_EDGE != 0 {
                b_temp = (b_temp & B_NOT_W_EDGE) << 1;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }
        }

        // Knight
        let b_attackers: u64 = b_opponent & self.b_knights;
        if b_attackers != 0 {
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0100) << 15) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0200) << 6) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0400) >> 10) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0500) >> 17) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0700) >> 15) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_0800) >> 6) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_1000) << 10) != 0 {
                return false;
            }
            if b_attackers & ((b_squares & B_KNIGHT_CAN_MOVE_1100) << 17) != 0 {
                return false;
            }
        }

        // King
        let b_attackers: u64 = b_opponent & self.b_kings;
        if ((b_squares & B_NOT_NE_EDGE) << 7) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_E_EDGE) >> 1) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_SE_EDGE) >> 9) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_S_EDGE) >> 8) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_SW_EDGE) >> 7) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_W_EDGE) << 1) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_NW_EDGE) << 9) & b_attackers != 0 {
            return false;
        }
        if ((b_squares & B_NOT_N_EDGE) << 8) & b_attackers != 0 {
            return false;
        }

        true
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
    fn is_safe_against_white_pawn_attacks() {
        let mut r = prep_board("8/p7/6p1/P3p3/8/7p/2P4P/8 w - - 0 100");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("g3".to_string())));
        assert!(r.black_is_safe(0xffffbfffffadffff)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_pawn_attacks() {
        let mut r = prep_board("8/p7/6p1/P3p3/8/7p/2P4P/8 w - - 0 100");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g2".to_string())));
        assert!(r.white_is_safe(0xffffbffaebfffdff)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_bishop_attacks() {
        let mut r = prep_board("8/4b3/8/8/1B2b3/8/6B1/6pp w - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h1".to_string())));
        assert!(r.black_is_safe(0xfff7ef5ff75aeff2)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_bishop_attacks() {
        let mut r = prep_board("8/4b3/8/8/1B2b3/8/6B1/6pp w - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b1".to_string())));
        assert!(r.white_is_safe(0x6bbec9c9beebddbf)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_diagonal_queen_attacks() {
        let mut r = prep_board("8/4q3/8/8/1Q2q3/8/6Q1/6kK w - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h1".to_string())));
    }

    #[test]
    fn is_safe_against_black_diagonal_queen_attacks() {
        let mut r = prep_board("8/4q3/8/8/1Q2q3/8/6Q1/6kK w - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b1".to_string())));
    }

    #[test]
    fn is_safe_against_white_rook_attacks() {
        let mut r = prep_board("r7/8/7R/8/8/8/8/2R4r b - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c8".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h8".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("g6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("g1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h1".to_string())));
        assert!(r.black_is_safe(0xdede01dededede20)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_rook_attacks() {
        let mut r = prep_board("r7/8/7R/8/8/8/8/2R4r b - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g1".to_string())));
        assert!(r.white_is_safe(0x807f7e7e7e7e7e41)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_straight_line_queen_attacks() {
        let mut r = prep_board("q7/8/7Q/8/8/8/8/2Q4q b - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c8".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h8".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("g6".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("d1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("g1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h1".to_string())));
    }

    #[test]
    fn is_safe_against_black_straight_line_queen_attacks() {
        let mut r = prep_board("q7/8/7Q/8/8/8/8/2Q4q b - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h4".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("a2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("d1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g1".to_string())));
    }

    #[test]
    fn is_safe_against_white_knight_attacks() {
        let mut r = prep_board("8/6n1/N7/8/3n4/6N1/8/8 w - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b8".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c7".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("c5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e4".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("e2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("f1".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("h1".to_string())));
        assert!(r.black_is_safe(0xbfdfffdab7fff7fa)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_knight_attacks() {
        let mut r = prep_board("8/6n1/N7/8/3n4/6N1/8/8 w - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e6".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h5".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("b3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("f3".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("c2".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("e2".to_string())));
        assert!(r.white_is_safe(0xf7ffd7baffbbd7ff)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_king_corner_attacks() {
        let mut r = prep_board("7k/8/8/8/8/8/8/K7 w - - 0 1");
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("a2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b2".to_string())));
        assert!(!r.black_is_safe(utils::convert_square_to_bitstring("b1".to_string())));
        assert!(r.black_is_safe(0xffffffffffff3fbf)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_king_corner_attacks() {
        let mut r = prep_board("7k/8/8/8/8/8/8/K7 w - - 0 1");
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g8".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("g7".to_string())));
        assert!(!r.white_is_safe(utils::convert_square_to_bitstring("h7".to_string())));
        assert!(r.white_is_safe(0xfdfcffffffffffff)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_king_castle_attack() {
        let mut r = prep_board("2k5/6pp/8/b7/8/P7/2P3PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2k5/6pp/7q/b7/8/PPPP4/6PP/R3K3 b Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2rk4/6pp/7q/b7/8/PP2P3/3P2PP/R3K3 b Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2rk4/6pp/7q/b7/8/PPP1P3/3Pp1PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2rk4/6pp/7q/b7/8/PPP1P3/1p1P2PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2rk4/5ppp/7q/b7/8/PPP1n3/3P2PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("2r2n2/5ppp/7q/b7/8/PPP5/1k1P2PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
        let mut r = prep_board("1kr5/5ppp/7q/b7/8/P1P5/n2P2PP/R3K3 w Q - 0 1");
        assert!(!r.white_is_safe(0x0000000000000038));
    }
}
