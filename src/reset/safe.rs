use crate::reset::Reset;

impl Reset {

    /// Determine whether whether a set of squares in a Reset is safe from an opponent's attack
    ///
    /// The `opponent` parameter will be '0' for white and '1' for black.  Returns `true` if the
    /// b_squares are safe and `false` otherwise.
    ///
    /// WARNING: This method does not work for the safety of pawns in an En Passant situation
    ///
    pub fn is_safe(&mut self, b_squares: u64, opponent: u8) -> bool {
        use crate::reset::r#const::B_NOT_UL_EDGE;
        use crate::reset::r#const::B_NOT_UR_EDGE;
        use crate::reset::r#const::B_NOT_DL_EDGE;
        use crate::reset::r#const::B_NOT_DR_EDGE;

        let b_opponent: u64 = if opponent == 0 {
            // Pawns - Down Left
            if ((b_squares & B_NOT_DL_EDGE) >> 7) & (self.b_pawns & self.b_white) != 0 {
                return false;
            }
            // Pawns - Down Right
            if ((b_squares & B_NOT_DR_EDGE) >> 9) & (self.b_pawns & self.b_white) != 0 {
                return false;
            }
            self.b_white
        } else {
            // Pawns - Up Left
            if ((b_squares & B_NOT_UL_EDGE) << 9) & (self.b_pawns & self.b_black) != 0 {
                return false;
            }
            // Pawns - Up Right
            if ((b_squares & B_NOT_UR_EDGE) << 7) & (self.b_pawns & self.b_black) != 0 {
                return false;
            }
            self.b_black
        };

        // Bishop or Queen
        let b_attackers: u64 = b_opponent & (self.b_bishops | self.b_queens);

        if b_attackers != 0 {
            // Bishop or Queen: Up Left
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_UL_EDGE != 0 {
                b_temp = (b_temp & B_NOT_UL_EDGE) << 9;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Up Right
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_UR_EDGE != 0 {
                b_temp = (b_temp & B_NOT_UR_EDGE) << 7;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Down Left
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_DL_EDGE != 0 {
                b_temp = (b_temp & B_NOT_DL_EDGE) >> 7;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
            }

            // Bishop or Queen: Down Right
            let mut b_temp: u64 = b_squares;
            while b_temp & B_NOT_DR_EDGE != 0 {
                b_temp = (b_temp & B_NOT_DR_EDGE) >> 9;
                if b_temp & b_attackers != 0 {
                    return false;
                }
                b_temp &= !(self.b_all);
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

    fn prep_board(fen: &str) -> Reset {
        let mut r = reset::new();
        let fen = String::from(fen);
        r.init_from_fen(fen);
        r
    }

    #[test]
    fn is_safe_against_white_pawn_attacks() {
        let mut r = prep_board("r7/p7/6p1/P3p3/8/7p/2P4P/8 w - - 0 100");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b6".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g3".to_string()),0));
        assert!(r.is_safe(0xffffbfffffadffff,0)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_pawn_attacks() {
        let mut r = prep_board("r7/p7/6p1/P3p3/8/7p/2P4P/8 w - - 0 100");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g2".to_string()),1));
        assert!(r.is_safe(0xffffbffaebfffdff,1)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_bishop_attacks() {
        let mut r = prep_board("8/4b3/8/8/1B2b3/8/6B1/6kK w - - 0 1");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e7".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d6".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a5".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c5".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e4".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d2".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e1".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f1".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h1".to_string()),0));
        assert!(r.is_safe(0xfff7ef5ff75aeff2,0)); // all safe squares
    }

    #[test]
    fn is_safe_against_black_bishop_attacks() {
        let mut r = prep_board("8/4b3/8/8/1B2b3/8/6B1/6kK w - - 0 1");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b7".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h7".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d3".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f3".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c2".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g2".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b1".to_string()),1));
        assert!(r.is_safe(0x6bbec9c9beebddbf,1)); // all safe squares
    }

    #[test]
    fn is_safe_against_white_diagonal_queen_attacks() {
        let mut r = prep_board("8/4q3/8/8/1Q2q3/8/6Q1/6kK w - - 0 1");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e7".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d6".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a5".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c5".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e4".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h3".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d2".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("e1".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f1".to_string()),0));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h1".to_string()),0));
    }

    #[test]
    fn is_safe_against_black_diagonal_queen_attacks() {
        let mut r = prep_board("8/4q3/8/8/1Q2q3/8/6Q1/6kK w - - 0 1");
        assert!(!r.is_safe(utils::convert_square_to_bitstring("a8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f8".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b7".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h7".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g6".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g5".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("h4".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("d3".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("f3".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("c2".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("g2".to_string()),1));
        assert!(!r.is_safe(utils::convert_square_to_bitstring("b1".to_string()),1));
    }

}
//  /* Bishop or Queen */
//  Temp = Squares;
//  Attackers = bBlack & (bBishops | bQueens);
//  while(Temp & ULEDGE)
//  {
//    Temp = (Temp & ULEDGE) >> 7;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & UREDGE)
//  {
//    Temp = (Temp & UREDGE) >> 9;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & DLEDGE)
//  {
//    Temp = (Temp & DLEDGE) << 9;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & DREDGE)
//  {
//    Temp = (Temp & DREDGE) << 7;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//
//  /* Rook or Queen */
//  Temp = Squares;
//  Attackers = bBlack & (bRooks | bQueens);
//  while(Temp & CANMOVEUP)
//  {
//    Temp = (Temp & CANMOVEUP) >> 8;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & CANMOVEDOWN)
//  {
//    Temp = (Temp & CANMOVEDOWN) << 8;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & CANMOVERIGHT)
//  {
//    Temp = (Temp & CANMOVERIGHT) >> 1;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  Temp = Squares;
//  while(Temp & CANMOVELEFT)
//  {
//    Temp = (Temp & CANMOVELEFT) << 1;
//    if (Temp & Attackers)
//      return FALSE;
//    Temp &= ~bAll;
//  }
//  
//  /* Knights */
//  Attackers = bKnights & bBlack;
//  if (Attackers & ((Squares & K0100) >> 17))
//    return FALSE;
//  if (Attackers & ((Squares & K0200) >> 10))
//    return FALSE;
//  if (Attackers & ((Squares & K0400) << 6))
//    return FALSE;
//  if (Attackers & ((Squares & K0500) << 15))
//    return FALSE;
//  if (Attackers & ((Squares & K0700) << 17))
//    return FALSE;
//  if (Attackers & ((Squares & K0800) << 10))
//    return FALSE;
//  if (Attackers & ((Squares & K1000) >> 6))
//    return FALSE;
//  if (Attackers & ((Squares & K1100) >> 15))
//    return FALSE;
//
//  /* King */
//  Attackers = bKings & bBlack;
//  if (((Squares & CANMOVEUP) >> 8) & Attackers)
//    return FALSE;
//  if (((Squares & CANMOVERIGHT) >> 1) & Attackers)
//    return FALSE;
//  if (((Squares & CANMOVELEFT) << 1) & Attackers)
//    return FALSE;
//  if (((Squares & CANMOVEDOWN) << 8) & Attackers)
//    return FALSE;
//  if (((Squares & DREDGE) << 7) & Attackers)
//    return FALSE;
//  if (((Squares & DLEDGE) << 9) & Attackers)
//    return FALSE;
//
//  return TRUE;
//}
