use crate::reset::Reset;
use crate::utils;

impl Reset {

    /// Prepare a Reset to generate moves
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// # assert_eq!(r.move_id,10);
    /// ```
    pub fn initialize_move_generation(&mut self) {
        if self.to_move == 0 {
            self.b_current_piece = bitops::lowest_bit(self.b_white);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black);
        }
        self.current_piece = utils:convert_bitstring_to_square(self.b_current_piece);
        self.move_id = 10;	//Prime the first move
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    #[test]
    fn move_init_move_generation() {
        let mut r = chessica::reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000001,"b_current_piece");
        assert_eq!(r.current_piece,1,"current_piece");
        assert_eq!(r.move_id,10,"move_id");

        let mut r = chessica::reset::new();
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        r.init_from_fen(fen2.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0001000000000000,"b_current_piece");
        assert_eq!(r.current_piece,49,"current_piece");
        assert_eq!(r.move_id,10,"move_id");
    }
}



/// Initialize move generation for a Reset
//void Reset::InitializeMoveGeneration()
//{
//}


/// Generate the next move for a Reset

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


