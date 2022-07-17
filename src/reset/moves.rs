use crate::reset::Reset;
use crate::bitops;

impl Reset {

    /// Prepare a Reset to generate moves
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn initialize_move_generation(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::lowest_bit(self.b_white);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black);
        }
        self.current_piece = bitops::get_bit_number(self.b_current_piece);
        self.move_id = 10;	//Prime the first move
    }

    /// Consider the next moveable piece
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn consider_next_moveable_piece(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::next_lowest_bit(self.b_white, self.b_current_piece);
        } else {
            self.b_current_piece = bitops::next_lowest_bit(self.b_black, self.b_current_piece);
        }
    }


    /// Generate the next move for a Reset
    ///
    /// Returns Boolean indicating `true` if move options have not been exhausted
    /// and `false` if they have.  Expects the child reset to already be initialized 
    /// from the parent.
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let mut child = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// r.generate_next_move(&mut child);
    /// ```
    pub fn generate_next_move(&mut self, child: &mut Reset) -> bool {
        while self.b_current_piece != 0 {
            if self.b_current_piece & self.b_pawns != 0 { // Pawn
                if self.generate_next_pawn_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_knights != 0 { // Knight
                if self.generate_next_knight_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_bishops != 0 { // Bishop
                if self.generate_next_bishop_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_rooks != 0 { // Rook
                if self.generate_next_rook_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_queens != 0 { // Queen
                if self.generate_next_queen_move(child) {
                    break;
                }
            } else { // King
                if self.generate_next_king_move(child) {
                    break;
                }
            }

            // do stuff
            self.consider_next_moveable_piece();
        }
        self.b_current_piece > 0
    }
}
////
////PERFORMANCE PATH
////
////Note: This gets called once per piecemove, not once per piece.  Queen moves should be second
//int Reset::GenerateNextMove(Reset *Target)
//{
//  int retcode = FALSE;
//
//  if (ToMove) 		//If it is WHITE's move
//  {
//    while (bCurrentPiece)
//    {
//      if (bCurrentPiece & bWhite)
//      {
//        if (bCurrentPiece & bPawns)
//        {
//          if (retcode = GenerateNextWhitePawnMove(Target))
//            break;
//        }
//        else
//        {
//          if (bCurrentPiece & bKnights)
//          {
//            if (retcode = GenerateNextWhiteKnightMove(Target))
//              break;
//          }
//          else
//          {
//            if (bCurrentPiece & bBishops)
//            {
//              if (retcode = GenerateNextWhiteBishopMove(Target))
//                break;
//            }
//            else
//            {
//              if (bCurrentPiece & bRooks)
//              {
//                if (retcode = GenerateNextWhiteRookMove(Target))
//                  break;
//              }
//              else
//              {
//                if (bCurrentPiece & bQueens)
//                {
//                  if (retcode = GenerateNextWhiteQueenMove(Target))
//                    break;
//                }
//                else //King
//                {
//                  if (retcode = GenerateNextWhiteKingMove(Target))
//                    break;
//                }
//              }
//            }
//          }
//        }
//      }
//      else
//      {
//        //Piece routines will advance bCurrentPiece by themselves
//        bCurrentPiece >>= 1;
//        CurrentPiece++;
//      }
//    }
//  }
//  else // Black's Move
//  {
//    while (bCurrentPiece)
//    {
//      if (bCurrentPiece & bBlack)
//      {
//        if (bCurrentPiece & bPawns)
//        {
//          if (retcode = GenerateNextBlackPawnMove(Target))
//            break;
//        }
//        else
//        {
//          if (bCurrentPiece & bKnights)
//          {
//            if (retcode = GenerateNextBlackKnightMove(Target))
//              break;
//          }
//          else
//          {
//            if (bCurrentPiece & bBishops)
//            {
//              if (retcode = GenerateNextBlackBishopMove(Target))
//                break;
//            }
//            else
//            {
//              if (bCurrentPiece & bRooks)
//              {
//                if (retcode = GenerateNextBlackRookMove(Target))
//                  break;
//              }
//              else
//              {
//                if (bCurrentPiece & bQueens)
//                {
//                  if (retcode = GenerateNextBlackQueenMove(Target))
//                    break;
//                }
//                else //King
//                {
//                  if (retcode = GenerateNextBlackKingMove(Target))
//                    break;
//                }
//              }
//            }
//          }
//        }
//      }
//      else
//      {
//        //Piece routines will advance bCurrentPiece by themselves
//        bCurrentPiece >>= 1;
//        CurrentPiece++;
//      }
//    }
//  }
//  return(retcode);
//}
//


#[cfg(test)]
mod tests {
    use crate::reset;

    #[test]
    fn move_init_move_generation() {
        let mut r = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000001,"b_current_piece");
        assert_eq!(r.current_piece,1,"current_piece");
        assert_eq!(r.move_id,10,"move_id");

        let mut r = reset::new();
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        r.init_from_fen(fen2.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0001000000000000,"b_current_piece");
        assert_eq!(r.current_piece,49,"current_piece");
        assert_eq!(r.move_id,10,"move_id");
    }

    #[test]
    fn move_consider_next_moveable_piece() {
        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 w Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000008);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000080);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);

        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 b Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0100000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0800000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);
    }
}

