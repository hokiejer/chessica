use crate::reset::Reset;
use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_SE_CORNER;
use crate::reset::r#const::B_SW_CORNER;
use crate::reset::r#const::B_NE_CORNER;

use crate::bitops::r#const::U8_NOT_BIT1;
use crate::bitops::r#const::U8_NOT_BIT2;
use crate::bitops::r#const::U8_NOT_BIT3;
use crate::bitops::r#const::U8_NOT_BIT4;

impl Reset {

    pub fn capture_processing(&mut self, child: &mut Reset) {
        child.capture = 1;
        child.halfmove_clock = 0; // Resets on capture

        child.b_all &= !child.b_to; // Useful for EP
        let material_multiplier: i8 = if self.white_to_move() {
            // Parent moved white
            1
        } else {
            // Parent moved black
            child.b_white &= !child.b_to;
            -1
        };

        if child.b_to & child.b_pawns != 0 {
            // Pawns
            child.b_pawns &= !child.b_to;
            child.material += material_multiplier;
            child.halfmove_clock = 0; // Resets on pawn move
        } else if child.b_to & child.b_knights != 0 {
            // Knights
            child.b_knights &= !child.b_to;
            child.material += material_multiplier * 3;
        } else if child.b_to & child.b_bishops != 0 {
            // Bishops
            child.b_bishops &= !child.b_to;
            child.material += material_multiplier * 3;
        } else if child.b_to & child.b_rooks != 0 {
            // Rooks
            child.b_rooks &= !child.b_to;
            child.material += material_multiplier * 5;
            if child.b_to & B_FOUR_CORNERS != 0 {
                if child.b_to & B_SE_CORNER != 0 {
                    //white_castle_k = 0;
                    child.castle_bits &= U8_NOT_BIT1;
                } else if child.b_to & B_SW_CORNER != 0 {
                    //white_castle_q = 0;
                    child.castle_bits &= U8_NOT_BIT2;
                } else if child.b_to & B_NE_CORNER != 0 {
                    //black_castle_k = 0;
                    child.castle_bits &= U8_NOT_BIT3;
                } else { // B_NW_CORNER
                    //black_castle_q = 0;
                    child.castle_bits &= U8_NOT_BIT4;
                }
            }
        } else {
            // Queens (Default - no king captures)
            child.material += material_multiplier * 9;
        }
    }
}
