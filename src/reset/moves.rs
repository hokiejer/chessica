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
        if self.to_move == 0 {
            self.b_current_piece = bitops::lowest_bit(self.b_white);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black);
        }
        self.current_piece = bitops::get_bit_number(self.b_current_piece);
        self.move_id = 10;	//Prime the first move
    }


    /// Generate the next move for a Reset
    ///
    /// Expects the child reset to already be initialized from
    /// the parent
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let mut child = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// r.generate_next_move(&mut child);
    /// # assert_eq!(0,1);
    /// ```
    pub fn generate_next_move(&mut self, child: &mut Reset) {
        if self.to_move == 0 {  // White's Move
            while self.b_current_piece != 0 {
                // do stuff
                self.b_current_piece = bitops::next_lowest_bit(self.b_white, self.b_current_piece); // Someday, I can create a Reset method for this called look_for_next_movable_piece
            }
        } else {    //Black's Move
            while self.b_current_piece != 0 {
                // do stuff
                self.b_current_piece = bitops::next_lowest_bit(self.b_black, self.b_current_piece); // Someday, I can create a Reset method for this called look_for_next_movable_piece
            }
        }
        todo!();
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
}

