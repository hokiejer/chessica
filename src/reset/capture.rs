use crate::reset::Reset;
use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_SE_CORNER;
use crate::reset::r#const::B_SW_CORNER;
use crate::reset::r#const::B_NE_CORNER;

impl Reset {

    pub fn capture_processing(&mut self) {
        self.capture = 1;
        self.halfmove_clock = 0; // Resets on capture

        self.b_all &= !self.b_to; // Useful for EP
        let material_multiplier: i8 = if self.white_to_move() {
            // Parent moved black
            self.b_white &= !self.b_to;
            -1
        } else {
            // Parent moved white
            1
        };

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
                if self.b_to & B_SE_CORNER != 0 {
                    //white_castle_k = 0;
                    self.castle_bits &= 0xfe;
                } else if self.b_to & B_SW_CORNER != 0 {
                    //white_castle_q = 0;
                    self.castle_bits &= 0xfd;
                } else if self.b_to & B_NE_CORNER != 0 {
                    //black_castle_k = 0;
                    self.castle_bits &= 0xfb;
                } else { // B_NW_CORNER
                    //black_castle_q = 0;
                    self.castle_bits &= 0xf7;
                }
            }
        } else {
            // Queens (Default - no king captures)
            self.material += material_multiplier * 9;
        }
    }
}
