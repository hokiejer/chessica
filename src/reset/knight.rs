use crate::reset::Reset;

pub const KNIGHT_CAN_MOVE_0100: u64 = 0x0000fefefefefefe;
pub const KNIGHT_CAN_MOVE_0200: u64 = 0x00fcfcfcfcfcfcfc;
pub const KNIGHT_CAN_MOVE_0400: u64 = 0xfcfcfcfcfcfcfc00;
pub const KNIGHT_CAN_MOVE_0500: u64 = 0xfefefefefefe0000;
pub const KNIGHT_CAN_MOVE_0700: u64 = 0x7f7f7f7f7f7f0000;
pub const KNIGHT_CAN_MOVE_0800: u64 = 0x3f3f3f3f3f3f3f00;
pub const KNIGHT_CAN_MOVE_1000: u64 = 0x003f3f3f3f3f3f3f;
pub const KNIGHT_CAN_MOVE_1100: u64 = 0x00007f7f7f7f7f7f;

/// Generate the next possible knight move
///
/// Returns `true` if a move was suggested, `false` otherwise.
///
/// # Examples
/// ```
/// ```
impl Reset {
    pub fn generate_next_knight_move(&mut self, child: &mut Reset) -> bool {
        
        if self.move_id == 10 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0100 != 0 {
                self.move_id = 20;
                return true;
            }
        }
        if self.move_id == 20 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0200 != 0 {
                self.move_id = 30;
                return true;
            }
        }
        if self.move_id == 30 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0400 != 0 {
                self.move_id = 40;
                return true;
            }
        }
        if self.move_id == 40 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0500 != 0 {
                self.move_id = 50;
                return true;
            }
        }
        if self.move_id == 50 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0700 != 0 {
                self.move_id = 60;
                return true;
            }
        }
        if self.move_id == 60 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_0800 != 0 {
                self.move_id = 70;
                return true;
            }
        }
        if self.move_id == 70 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_1000 != 0 {
                self.move_id = 80;
                return true;
            }
        }
        if self.move_id == 80 {
            if self.b_current_piece & KNIGHT_CAN_MOVE_1100 != 0 {
                self.consider_next_moveable_piece();
                self.move_id = 10;
                return true;
            }
        }
        self.consider_next_moveable_piece();
        self.move_id = 10;
        false
    }

}
