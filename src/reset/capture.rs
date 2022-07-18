use crate::reset::Reset;
use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_LOWER_RIGHT_CORNER;
use crate::reset::r#const::B_LOWER_LEFT_CORNER;
use crate::reset::r#const::B_UPPER_RIGHT_CORNER;
use crate::reset::r#const::B_UPPER_LEFT_CORNER;

impl Reset {

    pub fn capture_processing(&mut self) {
        let material_multiplier: i8;
        self.capture = 1;
        self.halfmove_clock = 0; // Resets on capture

        self.b_all &= !self.b_to; // Useful for EP
        if self.white_to_move() {
            self.b_black &= !self.b_to;
            material_multiplier = -1;
        } else {
            self.b_white &= !self.b_to;
            material_multiplier = 1;
        }

        if self.b_to & self.b_pawns != 0 {
            // Pawns
            self.b_pawns &= !self.b_to;
            self.material += material_multiplier;
            self.halfmove_clock = 0; // Resets on pawn move
        } else if self.b_to & self.b_knights != 0 {
            // Knights
            self.b_knights &= !self.b_to;
            self.material += material_multiplier * 3;
        } else if self.b_to & self.b_bishops != 0 {
            // Bishops
            self.b_bishops &= !self.b_to;
            self.material += material_multiplier * 3;
        } else if self.b_to & self.b_rooks != 0 {
            // Rooks
            self.b_rooks &= !self.b_to;
            self.material += material_multiplier * 5;
            if self.b_to & B_FOUR_CORNERS != 0 {
                if self.b_to & B_LOWER_RIGHT_CORNER != 0 {
                    self.white_castle_k = 0;
                } else if self.b_to & B_LOWER_LEFT_CORNER != 0 {
                    self.white_castle_q = 0;
                } else if self.b_to & B_UPPER_RIGHT_CORNER != 0 {
                    self.black_castle_k = 0;
                } else { // B_UPPER_RIGHT_CORNER
                    self.black_castle_q = 0;
                }
            }
        } else {
            // Queens (Default - no king captures)
            self.b_queens &= !self.b_to;
            self.material += material_multiplier * 9;
        }
    }
}
